// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashSet, time::Duration};

#[cfg(feature = "google")]
use lingual::{Lang, Translator};
#[cfg(feature = "google")]
use std::str::FromStr;

#[cfg(feature = "libretranslate")]
use libretranslate::{translate_url, Language};

use arboard::Clipboard;
use evdev::{Device, Key};
use tauri::Window;

fn contains_remove(set: &mut HashSet<Key>, keys: &[Key]) -> bool {
    let set_contains_key = keys.iter().all(|key| set.contains(key));

    if set_contains_key {
        keys.iter().for_each(|key| {
            set.remove(key);
        });
    }

    set_contains_key
}

static mut CLIPBOARD_INITED: bool = false;
static mut LISTENING_CLIPBOARD: bool = false;
#[cfg(feature = "google")]
static mut TRANSLATION_FROM_LANG: Lang = Lang::Fr;
#[cfg(feature = "libretranslate")]
static mut TRANSLATION_FROM_LANG: Language = Language::French;
#[cfg(feature = "google")]
static mut TRANSLATION_TO_LANG: Lang = Lang::En;
#[cfg(feature = "libretranslate")]
static mut TRANSLATION_TO_LANG: Language = Language::English;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct ClipboardEvent {
    source: String,
    translation: String,
}

#[tauri::command]
fn init_clipboard_reader(window: Window) {
    unsafe {
        if !CLIPBOARD_INITED {
            CLIPBOARD_INITED = true;
        } else {
            return;
        }
    }

    std::thread::spawn(move || {
        log::info!("clipboard reader started");

        #[cfg(feature = "google")]
        let translator = Translator::default();
        #[cfg(feature = "libretranslate")]
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let device = evdev::enumerate()
            .find_map(|(path, device)| -> Option<Device> {
                // check if the device has an ENTER key
                if device.supported_keys().map_or(false, |keys| {
                    keys.contains(Key::KEY_LEFTCTRL) && keys.contains(Key::KEY_C)
                }) {
                    log::info!("dev found its {}", path.display());
                    Some(device)
                } else {
                    log::info!("dev is not {} :(", path.display());
                    None
                }
            });
        let mut device = if let Some(d) = device {
            d
        } else {
            log::error!("no device found");
            return;
        };

        let mut pressed_keys: HashSet<Key> = HashSet::new();
        let mut clipboard = Clipboard::new().unwrap();

        window.emit("backend-ready", ()).unwrap();
        log::info!("clipboard backend ready");

        while let Ok(events) = device.fetch_events() {
            let events = events.collect::<Vec<_>>();

            for e in &events {
                if let evdev::InputEventKind::Key(key) = e.kind() {
                    if e.value() == 1 {
                        log::trace!("key pressed: {:?}", key);
                        pressed_keys.insert(key);
                    } else if e.value() == 0 {
                        log::trace!("key released: {:?}", key);
                        pressed_keys.remove(&key);
                    }
                }
            }

            unsafe {
                if !LISTENING_CLIPBOARD {
                    continue;
                }
            }

            if contains_remove(&mut pressed_keys, &[Key::KEY_LEFTCTRL, Key::KEY_C]) {
                log::debug!("clipboard hotkey triggered");

                std::thread::sleep(Duration::from_millis(50));
                log::trace!("slept for 50ms waiting for clipboard to update");

                if let Ok(res) = clipboard.get_text() {
                    log::debug!("got clipboard text: {}", res);

                    #[cfg(feature = "google")]
                    unsafe {
                        #[allow(static_mut_refs)]
                        let translation = translator
                            .translate(&res, &TRANSLATION_FROM_LANG, &TRANSLATION_TO_LANG)
                            .map(|res| res.text)
                            .unwrap_or_default();

                        log::debug!("got translation: {}", translation);
                        clipboard.set_text(&translation).ok();
                        window
                            .emit(
                                "clipboard-read",
                                ClipboardEvent {
                                    source: res,
                                    translation,
                                },
                            )
                            .unwrap();
                    }

                    #[cfg(feature = "libretranslate")]
                    unsafe {
                        runtime.block_on(async {
                            let translation = translate_url(
                                TRANSLATION_FROM_LANG,
                                TRANSLATION_TO_LANG,
                                &res,
                                &"https://translate.terraprint.co/".to_string(),
                                None,
                            )
                            .await
                            .map(|res| res.output)
                            .unwrap_or_default();

                            log::debug!("got translation: {}", translation);
                            clipboard.set_text(&translation).ok();
                            window
                                .emit(
                                    "clipboard-read",
                                    ClipboardEvent {
                                        source: res,
                                        translation,
                                    },
                                )
                                .unwrap();
                        });
                    }
                }
            }
        }
    });
}

#[tauri::command]
fn set_clipboard_reader(enabled: bool) {
    unsafe {
        LISTENING_CLIPBOARD = enabled;
        if enabled {
            log::debug!("listening to clipboard");
        } else {
            log::debug!("stopped listening to clipboard");
        }
    }
}

#[tauri::command]
fn set_source_language(source_language: String) {
    #[cfg(feature = "google")]
    let lang = Lang::from_str(&source_language);
    #[cfg(feature = "libretranslate")]
    let lang = Language::from(&source_language);

    if let Ok(lang) = lang {
        log::debug!("set target language to: {source_language}");
        unsafe {
            TRANSLATION_FROM_LANG = lang;
        }
    } else {
        log::warn!("Invalid language: {source_language}");
    }
}
#[tauri::command]
fn set_target_language(target_language: String) {
    #[cfg(feature = "google")]
    let lang = Lang::from_str(&target_language);
    #[cfg(feature = "libretranslate")]
    let lang = Language::from(&target_language);

    if let Ok(lang) = lang {
        log::debug!("set target language to: {target_language}");
        unsafe {
            TRANSLATION_TO_LANG = lang;
        }
    } else {
        log::warn!("Invalid language: {target_language}");
    }
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        #[cfg(debug_assertions)]
        std::env::set_var("RUST_LOG", "info");
        #[cfg(not(debug_assertions))]
        std::env::set_var("RUST_LOG", "warn");
    }
    pretty_env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            init_clipboard_reader,
            set_clipboard_reader,
            set_source_language,
            set_target_language,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
