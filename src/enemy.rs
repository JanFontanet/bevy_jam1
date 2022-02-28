use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{collide::Collideable, game::*};

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
        });
}

fn move_enemy(time: Res<Time>, mut enemy_transform: Query<(&mut Transform, &Speed), With<Enemy>>) {
    for (mut transform, velocity) in enemy_transform.iter_mut() {
        transform.translation.x += time.delta_seconds() * velocity.0;
    }
}
