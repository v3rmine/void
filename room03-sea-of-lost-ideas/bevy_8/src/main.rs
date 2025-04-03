use bevy::prelude::*;
use rhai::{CallFnOptions, Engine, Scope};

#[derive(Resource)]
struct Rhai {
    engine: Engine,
    scope: Scope<'static>,
    ast: rhai::AST,
    update_fn: &'static str,
}

fn main() {
    let mut engine = Engine::new();
    let ast = engine.compile_file("src/game.rhai".into()).unwrap();
    let scope = Scope::new();

    let has_30_fps_update = ast.iter_functions().any(|f| f.name == "_update");
    let has_60_fps_update = ast.iter_functions().any(|f| f.name == "_update60");
    assert!(
        has_30_fps_update != has_60_fps_update,
        "Either 30 or 60 FPS is required"
    );

    engine.register_fn("test", || {
        println!("Hello, world!");
    });

    App::new()
        .add_plugins(DefaultPlugins.build().set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (512.0, 512.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(Rhai {
            engine,
            scope,
            ast,
            update_fn: if has_30_fps_update {
                "_update"
            } else {
                "_update60"
            },
        })
        .insert_resource(Time::<Fixed>::from_hz(if has_30_fps_update {
            30.0
        } else {
            60.0
        }))
        .add_systems(Startup, on_startup)
        .add_systems(FixedUpdate, on_update)
        .add_systems(Update, on_draw)
        .run();
}

fn on_startup(mut rhai: ResMut<Rhai>) {
    let rhai = rhai.as_mut();
    rhai.engine
        .call_fn_with_options::<()>(
            CallFnOptions::new().rewind_scope(false),
            &mut rhai.scope,
            &rhai.ast,
            "_init",
            (),
        )
        .unwrap();
}

fn on_update(mut rhai: ResMut<Rhai>) {
    let rhai = rhai.as_mut();
    rhai.engine
        .call_fn_with_options::<()>(
            CallFnOptions::new().rewind_scope(false),
            &mut rhai.scope,
            &rhai.ast,
            &rhai.update_fn,
            (),
        )
        .unwrap();
}

fn on_draw(mut rhai: ResMut<Rhai>) {
    let rhai = rhai.as_mut();
    rhai.engine
        .call_fn_with_options::<()>(
            CallFnOptions::new().rewind_scope(false),
            &mut rhai.scope,
            &rhai.ast,
            "_draw",
            (),
        )
        .unwrap();
}
