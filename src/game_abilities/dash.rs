use crate::abilities::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct DashAbility;

#[derive(Bundle)]
pub struct DashAbilityBundle {
    pub marker: DashAbility, //marker,
    pub cooldown: Cooldown,
}
