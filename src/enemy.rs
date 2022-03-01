use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    bullet::Bullet,
    collide::{Collideable, DetectLeave},
    game::*,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_enemy));
    }
}

#[derive(Component)]
struct Enemy;

fn spawn_enemy(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 5,
        feature: shapes::RegularPolygonFeature::Radius(BASE_RADIUS),
        ..shapes::RegularPolygon::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::RED),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform {
                translation: Vec3::new(100.0, 0.0, 10.0),
                ..Default::default()
            },
        ))
        .insert(Enemy)
        .insert(Speed(BASE_SPEED))
        .insert(Collideable {
            radius: BASE_RADIUS,
        })
        .insert(DetectLeave);
}

fn move_enemy(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &Speed), With<Enemy>>,
    bullets_query: Query<&Transform, (With<Bullet>, Without<Enemy>)>,
) {
    if let Ok((mut transform, speed)) = enemy_query.get_single_mut() {
        let mut closest_bullet_transform: Option<&Transform> = None;
        let mut closest_distance: f32 = std::f32::MAX;
        for bullet_transform in bullets_query.iter() {
            let distance = bullet_transform.translation.distance(transform.translation);
            if distance < closest_distance {
                closest_distance = distance;
                closest_bullet_transform = Some(bullet_transform);
            }
        }
        if let Some(bullet) = closest_bullet_transform {
            let bullet_direction = bullet.translation.truncate().normalize();
            let movement_direction = Vec3::new(bullet_direction.y, -bullet_direction.x, 0.0);
            let magnitude = time.delta_seconds() * speed.0;
            transform.translation += magnitude * movement_direction;
        }
    }
}
