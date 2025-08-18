use core::{EmulatorIO, EmulatorTick, chip8::Chip8Emulator};
use std::{env, fs::File, io::Read, process::exit};

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};

mod core;

#[derive(Resource, Default)]
struct Emu(pub Chip8Emulator);

#[derive(Resource)]
struct CliArgs(pub Vec<String>);

fn main() {
    // Because connect_subsecond does not work outside of dx serve (which does not work with TUIs)
    dioxus_devtools::connect_at("ws://127.0.0.1:8080/_dioxus".to_string(), |msg| {
        if let dioxus_devtools::DevserverMsg::HotReload(hot_reload_msg) = msg {
            if let Some(jumptable) = hot_reload_msg.jump_table {
                unsafe { dioxus_devtools::subsecond::apply_patch(jumptable).unwrap() };
            }
        }
    });

    let frame_time = std::time::Duration::from_secs_f32(1. / 60.);

    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
            RatatuiPlugins::default(),
        ))
        .insert_resource(Emu::default())
        .insert_resource(CliArgs(env::args().collect::<Vec<String>>()))
        .add_systems(Startup, init)
        .add_systems(Update, tick_cpu)
        .add_systems(FixedUpdate, patched)
        .run();
}

fn init(mut emulator: ResMut<Emu>, args: Res<CliArgs>) {
    if args.0.len() != 2 {
        println!("Usage: cargo run path/to/game");
        exit(1);
    }

    let mut rom = File::open(&args.0[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    emulator.0.load(&buffer)
}

fn tick_cpu(mut emulator: ResMut<Emu>) {
    emulator.0.tick_cpu();
}

fn patched(mut context: ResMut<RatatuiContext>, emulator: ResMut<Emu>) {
    context.clear().unwrap();
    dioxus_devtools::subsecond::HotFn::current(draw_system)
        .call((context, emulator))
        .ok();
}

fn draw_system(mut context: ResMut<RatatuiContext>, mut emulator: ResMut<Emu>) -> Result {
    emulator.0.tick_frame();
    context.draw(|frame| {
        let text = ratatui::text::Text::raw("Hello world!");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}
