mod cooldown;

use crate::game::GameState;
use bevy::prelude::*;
pub use cooldown::*;
pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(tick_cooldowns));
    }
}

/// Marker component for Ability entities
#[derive(Component, Clone, Copy)]
pub struct Ability;
