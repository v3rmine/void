use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};

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
        .add_systems(Update, patched)
        .run();
}

fn patched(mut context: ResMut<RatatuiContext>) {
    context.clear().unwrap();
    dioxus_devtools::subsecond::HotFn::current(draw_system)
        .call((context,))
        .ok();
}

fn draw_system(mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let text = ratatui::text::Text::raw("Hello world!");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}
