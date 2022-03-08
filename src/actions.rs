use crate::direction::Direction;
use crate::game::GameState;
use crate::player::Player;
use crate::utils::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementEvent>()
            .add_event::<ShootEvent>()
            .init_resource::<ActionsMap>()
            .add_plugin(InputManagerPlugin::<Actions>::default())
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_movement_input.label("input"))
                    .with_system(handle_shoot_input.label("input")),
            );
    }
}

pub struct MovementEvent {
    pub direction: Direction,
}

pub struct ShootEvent {
    pub angle: f32,
}

pub struct ActionsMap {
    pub input_map: InputMap<Actions>,
}

impl Default for ActionsMap {
    fn default() -> Self {
        ActionsMap {
            input_map: Actions::get_default_input_map(),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Actions {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Shoot,
    // Dash,
}

impl Actions {
    pub const DIRECTIONS: [Self; 4] = [Actions::Up, Actions::Down, Actions::Left, Actions::Right];

    pub fn direction(self) -> Direction {
        match self {
            Actions::Up => Direction::UP,
            Actions::Down => Direction::DOWN,
            Actions::Left => Direction::LEFT,
            Actions::Right => Direction::RIGHT,
            _ => Direction::NEUTRAL,
        }
    }

    pub fn get_default_input_map() -> InputMap<Actions> {
        use Actions::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Up, KeyCode::W);

        input_map.insert(Down, KeyCode::S);

        input_map.insert(Left, KeyCode::A);

        input_map.insert(Right, KeyCode::D);

        // Abilities
        input_map.insert(Shoot, MouseButton::Left);

        input_map
    }
}

fn handle_movement_input(
    query: Query<&ActionState<Actions>, With<Player>>,
    mut event_writer: EventWriter<MovementEvent>,
) {
    let action_state = query.get_single();
    if let Err(err) = action_state {
        eprintln!("{:?}", err);
        return;
    }
    let action_state = action_state.unwrap();
    let mut direction = Direction::NEUTRAL;

    for input_direction in Actions::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            direction += input_direction.direction();
        }
    }

    if direction != Direction::NEUTRAL {
        event_writer.send(MovementEvent { direction });
    }
}

fn handle_shoot_input(
    windows: Res<Windows>,
    query: Query<(&ActionState<Actions>, &Transform), With<Player>>,
    mut event_writer: EventWriter<ShootEvent>,
) {
    let player = query.get_single();
    if let Err(err) = player {
        eprintln!("{:?}", err);
        return;
    }
    let (action_state, player_transform) = player.unwrap();
    if action_state.pressed(&Actions::Shoot) {
        let window = windows.get_primary().unwrap();

        if let Some(angle) = get_angle_between_transform_and_cursor(window, player_transform) {
            event_writer.send(ShootEvent { angle });
        }
    }
}
