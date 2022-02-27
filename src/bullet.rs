use crate::game::GameState;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_bullets));
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

pub(crate) fn create_bullet_bundle(transform: Transform, angle: f32, speed: f32) -> BulletBundle {
    let shape = shapes::Circle {
        radius: 8.,
        center: Vec2::ZERO,
    };

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

fn update_bullets(mut q_bullet: Query<(&mut Transform, &BulletAttributes), With<Bullet>>) {
    for (mut transform, attributes) in q_bullet.iter_mut() {
        transform.translation += attributes.speed
            * (Vec3::X * attributes.angle.cos() + Vec3::Y * attributes.angle.sin());
    }
}
