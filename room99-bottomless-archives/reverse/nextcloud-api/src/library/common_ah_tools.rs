use base64::{decode, encode};
#[cfg(debug_assertions)]
use headless_chrome::protocol::page::ScreenshotFormat;
use headless_chrome::Tab;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::env::var_os;
pub use std::env::{set_var, var};
#[cfg(debug_assertions)]
use std::io::Write;
use std::ops::Deref;
use std::time::Duration;

#[allow(dead_code)]
#[cfg(debug_assertions)]
fn take_screenshot(tab: &Tab, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("{}.png", path);
    let mut screen = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;
    std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new(path.as_str()))?)
        .write_all(screen.as_mut_slice())?;
    Ok(())
}
#[allow(dead_code)]
#[cfg(not(debug_assertions))]
fn take_screenshot(_tab: &Tab, _path: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[allow(dead_code)]
pub fn sleep(time: u64) {
    std::thread::sleep(Duration::from_millis(time));
}
#[allow(dead_code)]
pub fn urlencode<T: Deref<Target = str>>(url: T) -> String {
    let scopes = url.to_owned();
    scopes
        .replace(":", "%3A")
        .replace("/", "%2F")
        .replace(" ", "%20")
        .replace("?", "%3F")
        .replace("&", "%26")
        .replace("=", "%3D")
}
#[allow(dead_code)]
pub fn urldecode<T: Deref<Target = str>>(url: T) -> String {
    let scopes = url.to_owned();
    scopes
        .replace("%3A", ":")
        .replace("%2F", "/")
        .replace("%20", " ")
        .replace("%3F", "?")
        .replace("%26", "&")
        .replace("%3D", "=")
}
#[allow(dead_code)]
pub fn is_env(env: &str, exist: &dyn Fn(&str), notexist: &dyn Fn(&str)) {
    if var(env).is_err() || var(env).unwrap() == "" {
        if var_os(env).is_none() || var_os(env).unwrap() == "" {
            notexist(env);
        } else {
            exist(env);
        }
    } else {
        exist(env);
    }
}

pub fn serde_is_valid_and_contain<'a, T>(json: &'a str, key: &str, val: &str) -> bool
where
    T: Deserialize<'a>,
{
    let key = key.to_owned();
    let val = val.to_owned();
    let result = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(json).unwrap();
    serde_json::from_str::<'a, T>(json).is_ok()
        && result.contains_key(key.as_str())
        && result
            .get(key.as_str())
            .unwrap()
            .eq(&serde_json::Value::String(val))
}

type Error = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tomb {
    pub aes_key: String,
    pub aes_iv: String,
    pub value: String,
}

pub fn public_encrypt(
    key: &openssl::rsa::Rsa<openssl::pkey::Public>,
    bytes: &[u8],
) -> Result<Tomb, Error> {
    let mut sym_crypt = vec![0; key.size() as usize];
    let mut iv_crypt = vec![0; key.size() as usize];
    let (sym_key, iv, text) = sym_enc(String::from_utf8(bytes.to_vec())?)?;
    key.public_encrypt(
        &decode(&sym_key)?,
        &mut sym_crypt,
        openssl::rsa::Padding::PKCS1,
    )?;
    key.public_encrypt(&decode(&iv)?, &mut iv_crypt, openssl::rsa::Padding::PKCS1)?;
    Ok(Tomb {
        aes_key: encode(&sym_crypt),
        aes_iv: encode(&iv_crypt),
        value: text,
    })
}

pub fn private_decrypt(
    key: &openssl::rsa::Rsa<openssl::pkey::Private>,
    bytes: Tomb,
) -> Result<Tomb, Error> {
    let mut sym = vec![0; key.size() as usize];
    let mut iv = vec![0; key.size() as usize];
    let len_sym = key.private_decrypt(
        &decode(&bytes.aes_key)?,
        &mut sym,
        openssl::rsa::Padding::PKCS1,
    )?;
    let len_iv = key.private_decrypt(
        &decode(&bytes.aes_iv)?,
        &mut iv,
        openssl::rsa::Padding::PKCS1,
    )?;
    let sym = encode(&sym[..len_sym].to_vec());
    let iv = encode(&iv[..len_iv].to_vec());
    let text = sym_dec(sym.clone(), iv.clone(), bytes.value)?;
    Ok(Tomb {
        aes_key: sym,
        aes_iv: iv,
        value: encode(&text),
    })
}

fn sym_enc(msg: String) -> Result<(String, String, String), Error> {
    let rk = thread_rng().gen::<[u8; 16]>();
    let random = thread_rng().gen::<[u8; 16]>();

    let ciphertext = openssl::symm::encrypt(
        openssl::symm::Cipher::aes_128_ctr(),
        &rk,
        Some(&random),
        msg.as_bytes(),
    )
    .map_err(|e| e.to_string())?;

    Ok((
        encode(&rk.to_vec()),
        encode(&random.to_vec()),
        encode(&ciphertext),
    ))
}
fn sym_dec(key: String, iv: String, encrypted: String) -> Result<Vec<u8>, Error> {
    Ok(openssl::symm::decrypt(
        openssl::symm::Cipher::aes_128_ctr(),
        &decode(&key)?,
        Some(&decode(&iv)?),
        &decode(&encrypted)?,
    )
    .map_err(|e| e.to_string())?)
}
