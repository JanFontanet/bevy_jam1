use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    abilities::*,
    bullet::{create_bullet_bundle, BulletAttributes},
    collide::{Collideable, Collider, DetectLeave},
    game::*,
    game_abilities::*,
    player::{Player, PlayerBullet},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemy))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_enemy.label("movement"))
                    .with_system(shoot_action.after("movement")),
            );
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct EnemyBullet;

fn spawn_enemy(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 5,
        feature: shapes::RegularPolygonFeature::Radius(BASE_RADIUS),
        ..shapes::RegularPolygon::default()
    };

    let enemy = commands
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
        .insert(DetectLeave)
        .id();

    let shoot_ability = commands
        .spawn_bundle(ShootAbilityBundle {
            marker: ShootAbility,
            cooldown: Cooldown::new(0.3),
        })
        .insert(Ability)
        .id();

    commands.entity(enemy).push_children(&[shoot_ability]);
}

struct ClosestBullet {
    distance: f32,
    good_question: Vec2,
}

fn move_enemy(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &Speed), With<Enemy>>,
    bullets_query: Query<(&Transform, &BulletAttributes), (With<PlayerBullet>, Without<Enemy>)>,
) {
    if let Ok((mut transform, speed)) = enemy_query.get_single_mut() {
        let mut dangerous_bullets: Vec<ClosestBullet> = Vec::new();
        let mut closest_distance = std::f32::MAX;
        for (bullet_transform, attributes) in bullets_query.iter() {
            let distance = bullet_transform.translation.distance(transform.translation);

            let relative_position =
                (transform.translation - bullet_transform.translation).truncate();
            let relative_angle = Vec2::X.angle_between(relative_position);
            let radius = BASE_RADIUS + 12.;
            let transformed_angle = attributes.angle - relative_angle;
            if transformed_angle.abs() < (radius / (distance.powi(2) + radius.powi(2)).sqrt())
                || distance < BASE_RADIUS + 15.
            {
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
                if distance < closest_distance {
                    closest_distance = distance;
                }
            }
        }
        dangerous_bullets.retain(|bullet| bullet.distance < closest_distance * 2.);

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

fn shoot_action(
    mut commands: Commands,
    q_enemy: Query<(&Transform, &Children), With<Enemy>>,
    q_player: Query<&Transform, With<Player>>,
    mut q_child: Query<&mut Cooldown>,
) {
    if let Ok((enemy_transform, children)) = q_enemy.get_single() {
        if let Ok(player_transform) = q_player.get_single() {
            let mut bullet_transform = enemy_transform.clone();
            bullet_transform.translation.z -= 1.;
            let direction =
                player_transform.translation.truncate() - enemy_transform.translation.truncate();
            let angle = Vec2::X.angle_between(direction);
            let mut cd = q_child.get_mut(children[0]).unwrap();
            if !cd.finished() {
                return;
            }
            commands
                .spawn_bundle(create_bullet_bundle(
                    bullet_transform,
                    angle,
                    BULLET_SPEED,
                    Color::ORANGE_RED,
                ))
                .insert(Collider { radius: 8.0 })
                .insert(EnemyBullet);
            cd.start();
        }
    }
}
