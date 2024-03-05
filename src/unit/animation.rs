use std::f32::consts::PI;
use bevy::prelude::*;

use super::{component, State};

fn angle_to_direction(angle: f32) -> usize {
    let angle_positive = (angle + (2.0 * PI)) % (2.0 * PI);
    let index = ((angle_positive + PI / 8.0) % (2.0 * PI) / (PI / 4.0)) as usize;
    index
}

pub fn animate_texture_atlas(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut component::AnimationIndices, &mut component::AnimationTimer, &component::Facing, &component::CurrentState), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    for (mut atlas, mut indices, mut timer, facing, state) in query.iter_mut() {
        let direction = angle_to_direction(facing.value);
        if state.value != State::Move {
            atlas.index = direction * 8;
            continue;
        }
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            indices.current += 1;
            if indices.current > indices.last {
                indices.current = indices.first;
            }
            atlas.index = direction * 8 + indices.current;
        }
    }
}
