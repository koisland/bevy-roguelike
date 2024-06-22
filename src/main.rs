use bevy::prelude::*;

mod assets;
mod globals;
mod states;
mod vectors;

mod board;
mod camera;
mod graphics;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(Msaa::Off)
        .add_state::<states::MainState>()
        .add_plugin(board::BoardPlugin)
        .add_plugin(assets::AssetPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_startup_system(camera::setup)
        .run()
}
