mod components;
mod plugins;
mod prelude;
mod resources;
mod systems;

use {
    bevy::{
        prelude::*,
        window::{WindowPosition, WindowResolution},
    },
    bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter},
    plugins::{food::FoodPlugin, player::PlayerPlugin, tile::TilePlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(656., 656.),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(FramepacePlugin)
        .add_plugin(TilePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(FoodPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_systems((cap_fps, spawn_camera))
        .run();
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());
}

fn cap_fps(mut fps_settings: ResMut<FramepaceSettings>) {
    fps_settings.limiter = Limiter::from_framerate(15.);
}
