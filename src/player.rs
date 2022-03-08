use crate::abilities::{Ability, Cooldown};
use crate::actions::{Actions, ActionsMap, MovementEvent, ShootEvent};
use crate::bullet::create_bullet_bundle;
use crate::collide::{Collideable, Collider, DetectLeave};
use crate::game::{GameState, Speed, BASE_RADIUS, BASE_SPEED, BULLET_SPEED};
use crate::shoot::{ShootAbility, ShootAbilityBundle};
use crate::utils::*;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use leafwing_input_manager::prelude::*;
use std::f32::consts::PI;

const PLAYER_BASE_ANGLE: f32 = -PI / 2.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(cursor_system)
                    .with_system(handle_movement_events.after("input").label("movement"))
                    .with_system(handle_shoot_events.after("input").label("action"))
                    .with_system(go_to_menu_again),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerBullet;

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

    let player = commands
        .spawn_bundle(PlayerBundle {
            player: Player,
            shape: GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::PURPLE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.0),
                },
                Transform {
                    translation: Vec3::new(-350.0, 0.0, 10.0),
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
        .insert(Speed(BASE_SPEED))
        .id();

    let shoot_ability = commands
        .spawn_bundle(ShootAbilityBundle {
            marker: ShootAbility,
            cooldown: Cooldown::new(0.3),
        })
        .insert(Ability)
        .id();

    commands.entity(player).push_children(&[shoot_ability]);
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
    q_player: Query<(&Transform, &Children), With<Player>>,
    mut q_ability: Query<&mut Cooldown, With<ShootAbility>>,
) {
    let player_transform = q_player.get_single();
    if let Err(err) = player_transform {
        eprintln!("{:?}", err);
        return;
    }
    let (player_transform, children) = player_transform.unwrap();

    let mut bullet_transform = player_transform.clone();
    bullet_transform.translation.z -= 1.;

    for event in events.iter() {
        let mut cd = q_ability.get_mut(children[0]).unwrap();
        if !cd.finished() {
            return;
        }
        commands
            .spawn_bundle(create_bullet_bundle(
                bullet_transform,
                event.angle,
                BULLET_SPEED,
                Color::ORANGE,
            ))
            .insert(Collider { radius: 8.0 })
            .insert(PlayerBullet);
        cd.start();
    }
}

fn go_to_menu_again(mut state: ResMut<State<GameState>>, q_player: Query<&Player>) {
    if q_player.is_empty() {
        state.set(GameState::Menu).unwrap();
    }
}
