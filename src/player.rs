// mod direction;
use crate::actions::{Actions, ActionsMap, MovementEvent, ShootEvent};
use crate::game::GameState;
use crate::utils::*;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
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
        feature: shapes::RegularPolygonFeature::Radius(20.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(PlayerBundle {
        player: Player,
        shape: GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::PURPLE),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform::default(),
        ),
        input_manager: InputManagerBundle {
            action_state: ActionState::default(),
            input_map: actions_map.input_map.clone(),
        },
    });
}

fn cursor_system(windows: Res<Windows>, mut q_player: Query<&mut Transform, With<Player>>) {
    let window = windows.get_primary().unwrap();
    let mut player_transform = q_player.single_mut();

    if let Some(direction) = get_direction_between_transform_and_cursor(window, &player_transform) {
        let angle = direction.angle_between(Vec2::new(0., 1.));

        player_transform.rotation = Quat::from_rotation_z(-angle);
    }
}

fn handle_movement_events(
    mut events: EventReader<MovementEvent>,
    mut q_player: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = q_player.single_mut();
    for event in events.iter() {
        player_transform.translation += Vec3::from(event.direction);
    }
}

fn handle_shoot_events(
    mut commands: Commands,
    mut events: EventReader<ShootEvent>,
    mut q_player: Query<&Transform, With<Player>>,
) {
    let mut player_transform = q_player.single_mut();
    for event in events.iter() {
        //spawn bullet? (atack speed?)
    }
}
