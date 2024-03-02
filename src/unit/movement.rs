use bevy::prelude::*;

use super::{component, component::AsVec2, action::Action};

const ARRIVAL_DISTANCE: f32 = 1.0;

pub fn direct_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut component::Velocity), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
        velocity.x = 0.0;
        velocity.y = 0.0;
    }
}

pub fn calculate_direct_velocity(
    mut query: Query<(&mut component::Velocity, &component::MoveSpeed, &component::State, &component::Target, &Transform), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    for (mut velocity, move_speed, state, target, transform) in query.iter_mut() {
        if state.value != Action::Move {
            continue;
        }
        let direction = (target.as_vec2() - transform.translation.truncate()).normalize();
        velocity.x = direction.x * move_speed.value;
        velocity.y = direction.y * move_speed.value;
    }
}

pub fn arrive(
    mut query: Query<(&mut component::Velocity, &mut component::State, &component::Target, &Transform), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    for (mut velocity, mut state, target, transform) in query.iter_mut() {
        if state.value != Action::Move {
            continue;
        }
        let distance = transform.translation.xy().distance(target.as_vec2());
        if distance < ARRIVAL_DISTANCE {
            state.value = Action::Idle;
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

fn face_point(
    transform: &mut Transform,
    turn_amount: f32,
    point: Vec3,
    ) {
    let forward = (transform.rotation * Vec3::Y).truncate();
    let to_target = (point - transform.translation).truncate().normalize();
    let forward_dot_target = forward.dot(to_target);

    if (forward_dot_target - 1.0).abs() < f32::EPSILON {
        return;
    }

    let right = (transform.rotation * Vec3::X).truncate();
    let right_dot_target = right.dot(to_target);
    let rotation_sign = -f32::copysign(1.0, right_dot_target);
    let max_angle = forward_dot_target.clamp(-1.0, 1.0).acos();
    let rotation_angle = rotation_sign * turn_amount.min(max_angle);
    transform.rotate_z(rotation_angle);
}

pub fn turn_towards_target(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &component::TurnRate, &component::Target), With<component::Unit>>,
    ) {
    for (mut transform, turn_rate, target) in query.iter_mut() {
        let turn_amount = turn_rate.value * time.delta_seconds();
        face_point(&mut transform, turn_amount, target.as_vec2().extend(0.0));
    }
}
