use crate::game::GameState;
use bevy::prelude::*;

pub struct CollidePlugin;

impl Plugin for CollidePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityLeaveWindow>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(collide_system)
                .with_system(detect_entity_leaving),
        );
    }
}

#[derive(Component)]
pub struct Collideable {
    pub radius: f32,
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

pub struct EntityLeaveWindow {
    pub entity: Entity,
    pub last_x: f32,
    pub last_y: f32,
}
#[derive(Component)]
pub struct DetectLeave;

fn collide_system(
    mut commands: Commands,
    q_collidables: Query<(Entity, &Transform, &Collideable), With<Collideable>>,
    q_colliders: Query<(Entity, &Transform, &Collider), With<Collider>>,
) {
    for (ent1, collidable_transform, collidable) in q_collidables.iter() {
        for (ent2, collider_transform, collider) in q_colliders.iter() {
            if ent1 == ent2 {
                continue;
            }
            let objects_distance = collidable_transform
                .translation
                .distance(collider_transform.translation);
            if objects_distance < collidable.radius + collider.radius {
                commands.entity(ent1).despawn_recursive();
                commands.entity(ent2).despawn_recursive();
            }
        }
    }
}

fn detect_entity_leaving(
    windows: Res<Windows>,
    query: Query<(Entity, &Transform), With<DetectLeave>>,
    mut event_writer: EventWriter<EntityLeaveWindow>,
) {
    let window = windows.get_primary().unwrap();
    for (entity, transform) in query.iter() {
        let (x, y) = (transform.translation.x, transform.translation.y);
        let (width, height) = (window.width(), window.height());
        if x > width / 2. {
            event_writer.send(EntityLeaveWindow {
                entity,
                last_x: width / 2.,
                last_y: y,
            });
        } else if x < -width / 2. {
            event_writer.send(EntityLeaveWindow {
                entity,
                last_x: -width / 2.,
                last_y: y,
            });
        } else if y > height / 2. {
            event_writer.send(EntityLeaveWindow {
                entity,
                last_x: x,
                last_y: height / 2.,
            });
        } else if y < -height / 2. {
            event_writer.send(EntityLeaveWindow {
                entity,
                last_x: x,
                last_y: -height / 2.,
            });
        }
    }
}
