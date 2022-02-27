mod actions;
mod direction;
mod game;
mod loading;
mod menu;
mod player;
mod utils;

//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::WindowMode};
use game::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0., 0.1, 0.3)))
        .insert_resource(WindowDescriptor {
            title: "Shooting Cubes".to_string(), // ToDo
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
