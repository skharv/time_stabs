use bevy::prelude::*;

use super::{component, component::AsVec2, State};

const ARRIVAL_DISTANCE: f32 = 5.0;

pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut component::Velocity), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
        transform.translation.z = -transform.translation.y;
        velocity.x = 0.0;
        velocity.y = 0.0;
    }
}

pub fn calculate_direct_velocity(
    mut query: Query<(&mut component::Velocity, &component::MoveSpeed, &component::Facing, &component::CurrentState), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    for (mut velocity, move_speed, facing, state) in query.iter_mut() {
        if state.value != State::Move {
            continue;
        }
        let direction = Vec2::new(facing.value.cos(), facing.value.sin()).normalize();
        velocity.x = direction.x * move_speed.value;
        velocity.y = direction.y * move_speed.value;
    }
}

pub fn arrive(
    mut query: Query<(&mut component::Velocity, &mut component::CurrentState, &component::Target, &Transform), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    for (mut velocity, mut state, target, transform) in query.iter_mut() {
        if state.value != State::Move {
            continue;
        }
        let distance = transform.translation.xy().distance(target.as_vec2());
        if distance < ARRIVAL_DISTANCE {
            state.value = State::Idle;
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

pub fn rotate_facing(
    current_point: Vec3,
    facing: f32,
    turn_amount: f32,
    target_point: Vec3,
    ) -> Result<f32, String> {
    let forward = Vec2::new(facing.cos(), facing.sin()).normalize();
    let to_target = (target_point - current_point).truncate().normalize();
    let forward_dot_target = forward.dot(to_target);

    if (forward_dot_target - 1.0).abs() < f32::EPSILON {
        return Err("no rotation needed".to_string());
    }

    let right = Vec2::new(-facing.sin(), facing.cos()).normalize();
    let right_dot_target = right.dot(to_target);
    let rotation_sign = -f32::copysign(1.0, right_dot_target);
    let max_angle = forward_dot_target.clamp(-1.0, 1.0).acos();
    let rotation_angle = rotation_sign * turn_amount.min(max_angle);
    Ok(rotation_angle)
}

pub fn turn_towards_target(
    time: Res<Time>,
    mut query: Query<(&Transform, &component::TurnRate, &mut component::Facing, &component::Target, &component::CurrentState), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    for (transform, turn_rate, mut facing, target, state) in query.iter_mut() {
        let turn_amount = turn_rate.value * time.delta_seconds();
        if let Ok(face) = rotate_facing(transform.translation, facing.value, turn_amount, target.as_vec2().extend(0.0)) {
            facing.value -= face;
        }
    }
}
