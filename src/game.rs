use crate::actions::*;
use crate::bullet::*;
use crate::collide::CollidePlugin;
use crate::enemy::*;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::*;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(ShapePlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(CollidePlugin)
            .add_plugin(EnemyPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
pub struct Speed(pub f32);

pub const BASE_SPEED: f32 = 50.;
pub const BASE_RADIUS: f32 = 20.;

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
