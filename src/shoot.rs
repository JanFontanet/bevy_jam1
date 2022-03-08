use crate::abilities::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ShootAbility;

#[derive(Bundle)]
pub struct ShootAbilityBundle {
    pub marker: ShootAbility, //marker,
    pub cooldown: Cooldown,
}
