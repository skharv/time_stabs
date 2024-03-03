use bevy::prelude::*;

use crate::unit::component;
use super::component::Bullet;

pub fn calculate_and_apply_velocity(
    mut query: Query<(&mut component::Velocity, &component::MoveSpeed, &mut Transform), With<Bullet>>,
    time: Res<Time>,
    ) {
    for (mut velocity, move_speed, mut transform) in query.iter_mut() {
        let forward = (transform.rotation * Vec3::Y).truncate();
        velocity.x = forward.x * move_speed.value * time.delta_seconds();
        velocity.y = forward.y * move_speed.value * time.delta_seconds();
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
