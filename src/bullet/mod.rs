use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use crate::unit::component::{Velocity, MoveSpeed};

mod component;
mod movement;

const BULLET_SPEED: f32 = 500.0;
const BULLET_RADIUS: f32 = 5.0;
const BULLET_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

#[derive(Event)]
pub struct Fire(pub Vec2, pub f32);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fire, movement::calculate_and_apply_velocity))
            .add_event::<Fire>();
    }
}

pub fn fire(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut reader: EventReader<Fire>,
    ) {
    for event in reader.read() {
        let mut spawn_transform = Transform::from_xyz(event.0.x, event.0.y, 0.0);
        spawn_transform.rotate_z(event.1);
        let forward = (spawn_transform.rotation * Vec3::Y).truncate();
        commands.spawn((MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: BULLET_RADIUS })),
            material: materials.add(BULLET_COLOR),
            transform: spawn_transform,
            ..default()
        },
        component::Bullet,
        Velocity { x: forward.x, y: forward.y },
        MoveSpeed { value: BULLET_SPEED },
        ));
    }
}
