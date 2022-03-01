use crate::actions::{Actions, ActionsMap, MovementEvent, ShootEvent};
use crate::bullet::create_bullet_bundle;
use crate::collide::{Collideable, Collider, DetectLeave};
use crate::game::{GameState, Speed, BASE_RADIUS, BASE_SPEED};
use crate::utils::*;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use leafwing_input_manager::prelude::*;
use std::f32::consts::PI;

const PLAYER_BASE_ANGLE: f32 = -PI / 2.0;
const BULLET_SPEED: f32 = 200.0; // NOTE: points per seccond

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(cursor_system)
                    .with_system(handle_movement_events.after("input").label("movement"))
                    .with_system(handle_shoot_events.after("input").label("action")),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    #[bundle]
    pub shape: ShapeBundle,
    #[bundle]
    pub input_manager: InputManagerBundle<Actions>,
}

fn spawn_player(mut commands: Commands, actions_map: Res<ActionsMap>) {
    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(BASE_RADIUS),
        ..shapes::RegularPolygon::default()
    };

    commands
        .spawn_bundle(PlayerBundle {
            player: Player,
            shape: GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::PURPLE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.0),
                },
                Transform {
                    translation: Vec3::new(-100.0, 0.0, 10.0),
                    ..Default::default()
                },
            ),
            input_manager: InputManagerBundle {
                action_state: ActionState::default(),
                input_map: actions_map.input_map.clone(),
            },
        })
        .insert(Collideable {
            radius: BASE_RADIUS,
        })
        .insert(DetectLeave)
        .insert(Speed(BASE_SPEED));
}

fn cursor_system(windows: Res<Windows>, mut q_player: Query<&mut Transform, With<Player>>) {
    let window = windows.get_primary().unwrap();
    let player_transform = q_player.get_single_mut();
    if let Err(err) = player_transform {
        eprintln!("{:?}", err);
        return;
    }
    let mut player_transform = player_transform.unwrap();

    if let Some(angle) = get_angle_between_transform_and_cursor(window, &player_transform) {
        player_transform.rotation = Quat::from_rotation_z(angle + PLAYER_BASE_ANGLE);
    }
}

fn handle_movement_events(
    mut events: EventReader<MovementEvent>,
    mut q_player: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time>,
) {
    let player = q_player.get_single_mut();
    if let Err(err) = player {
        eprintln!("{:?}", err);
        return;
    }
    let (mut player_transform, speed) = player.unwrap();

    for event in events.iter() {
        player_transform.translation +=
            Vec3::from(event.direction) * time.delta_seconds() * speed.0;
    }
}

fn handle_shoot_events(
    mut commands: Commands,
    mut events: EventReader<ShootEvent>,
    q_player: Query<&Transform, With<Player>>,
) {
    let player_transform = q_player.get_single();
    if let Err(err) = player_transform {
        eprintln!("{:?}", err);
        return;
    }
    let player_transform = player_transform.unwrap();
    let mut bullet_transform = player_transform.clone();
    bullet_transform.translation.z -= 1.;

    for event in events.iter() {
        commands
            .spawn_bundle(create_bullet_bundle(
                bullet_transform,
                event.angle,
                BULLET_SPEED,
            ))
            .insert(Collider { radius: 8.0 });
    }
}
