use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};

fn main() {
    dioxus_devtools::connect_subsecond();
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
        let text = ratatui::text::Text::raw(format!("hello {}", frame.count()));
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}
