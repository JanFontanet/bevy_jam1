mod abilities;
mod actions;
mod bullet;
mod collide;
mod direction;
mod enemy;
mod game;
mod game_abilities;
mod loading;
mod menu;
mod player;
mod utils;

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
