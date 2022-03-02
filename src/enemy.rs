use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    bullet::{Bullet, BulletAttributes},
    collide::{Collideable, DetectLeave},
    game::*,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemy))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_enemy)
                    .label("movement"),
            );
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
                translation: Vec3::new(350.0, 0.0, 10.0),
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

struct ClosestBullet {
    distance: f32,
    good_question: Vec2,
}

fn move_enemy(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &Speed), With<Enemy>>,
    bullets_query: Query<(&Transform, &BulletAttributes), (With<Bullet>, Without<Enemy>)>,
) {
    if let Ok((mut transform, speed)) = enemy_query.get_single_mut() {
        let mut dangerous_bullets: Vec<ClosestBullet> = Vec::new();
        for (bullet_transform, attributes) in bullets_query.iter() {
            let distance = bullet_transform.translation.distance(transform.translation);

            let relative_position =
                (transform.translation - bullet_transform.translation).truncate();
            let relative_angle = Vec2::X.angle_between(relative_position);
            let radius = BASE_RADIUS + 12.;
            let transformed_angle = attributes.angle - relative_angle;
            if transformed_angle.abs() < (radius / (distance.powi(2) + radius.powi(2)).sqrt()) {
                let x_sign: f32 = if transformed_angle.is_sign_positive() {
                    -1.
                } else {
                    1.
                };
                dangerous_bullets.push(ClosestBullet {
                    distance,
                    good_question: Vec2::new(
                        attributes.angle.sin(),
                        x_sign * attributes.angle.cos(),
                    ),
                });
            }
        }
        // dangerous_bullets.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        let mut inverted_distances = dangerous_bullets
            .iter()
            .map(|bullet| 1. / bullet.distance)
            .collect::<Vec<_>>();
        let sum_inv_distances = inverted_distances.iter().sum::<f32>();
        inverted_distances.iter_mut().for_each(|inv_distance| {
            *inv_distance /= sum_inv_distances;
        });
        let mut direction = Vec2::ZERO;
        dangerous_bullets
            .iter()
            .zip(inverted_distances.iter())
            .for_each(|(bullet, inv_distance)| {
                direction += bullet.good_question * *inv_distance;
            });
        transform.translation += direction.extend(0.) * speed.0 * time.delta_seconds();
    }
}
