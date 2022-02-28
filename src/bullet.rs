use crate::game::GameState;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_bullets)
                .with_system(handle_bullet_leave_window_events),
        );
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletAttributes {
    pub angle: f32,
    pub speed: f32,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub bullet_attributes: BulletAttributes,
    #[bundle]
    pub shape: ShapeBundle,
}

pub(crate) fn create_bullet_bundle(
    mut transform: Transform,
    angle: f32,
    speed: f32,
) -> BulletBundle {
    let shape = shapes::Circle {
        radius: 8.,
        center: Vec2::ZERO,
    };

    transform.translation += 30. * (Vec3::X * angle.cos() + Vec3::Y * angle.sin());

    BulletBundle {
        bullet: Bullet,
        bullet_attributes: BulletAttributes { angle, speed },
        shape: GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::ORANGE),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            transform,
        ),
    }
}

fn update_bullets(
    mut q_bullet: Query<(&mut Transform, &BulletAttributes), With<Bullet>>,
    time: Res<Time>,
) {
    for (mut transform, attributes) in q_bullet.iter_mut() {
        transform.translation += attributes.speed
            * time.delta_seconds()
            * (Vec3::X * attributes.angle.cos() + Vec3::Y * attributes.angle.sin());
    }
}

// fn handle_bullet_leave_window_events(
//     mut commands: Commands,
//     mut events: EventReader<EntityLeaveWindow>,
//     q_bullets: Query<Entity, With<Bullet>>,
// ) {
//     for entity in q_bullets.iter() {
//         for event in events.iter() {
//             if event.entity == entity {
//                 eprintln!("Bullet {:?} left window", entity);
//                 commands.entity(entity).despawn();
//             }
//         }
//     }
// }

fn handle_bullet_leave_window_events(
    mut commands: Commands,
    q_bullets: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (ent, bullet) in q_bullets.iter() {
        if bullet.translation.x.abs() > 2000. || bullet.translation.y.abs() > 1500. {
            commands.entity(ent).despawn();
        }
    }
}
