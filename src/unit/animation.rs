use std::f32::consts::PI;
use bevy::prelude::*;

use super::{component, State, action::Action};

fn angle_to_direction(angle: f32) -> usize {
    let angle_positive = (angle + (2.0 * PI)) % (2.0 * PI);
    let index = ((angle_positive + PI / 8.0) % (2.0 * PI) / (PI / 4.0)) as usize;
    index
}

pub fn animate_texture_atlas(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut component::AnimationIndices, &mut component::AnimationTimer, &component::Facing, &component::CurrentState, &component::CurrentAction), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    for (mut atlas, mut indices, mut timer, facing, state, action) in query.iter_mut() {
        let direction = angle_to_direction(facing.value);
        match state.value {
            State::Move => {
                timer.timer.tick(time.delta());
                if timer.timer.finished() {
                    indices.current += 1;
                    if indices.current > indices.last {
                        indices.current = indices.first;
                    }
                    atlas.index = direction * 8 + indices.current;
                }
            }
            State::Attack => {
                match direction {
                    0 => {
                        if action.value == Action::Attack {
                            atlas.index = 65;
                        } else {
                            atlas.index = 64;
                        }
                    }
                    1 => {
                        if action.value == Action::Attack {
                            atlas.index = 67;
                        } else {
                            atlas.index = 66;
                        }
                    }
                    2 => {
                        if action.value == Action::Attack {
                            atlas.index = 69;
                        } else {
                            atlas.index = 68;
                        }
                    }
                    3 => {
                        if action.value == Action::Attack {
                            atlas.index = 71;
                        } else {
                            atlas.index = 70;
                        }
                    }
                    4 => {
                        if action.value == Action::Attack {
                            atlas.index = 73;
                        } else {
                            atlas.index = 72;
                        }
                    }
                    5 => {
                        if action.value == Action::Attack {
                            atlas.index = 75;
                        } else {
                            atlas.index = 74;
                        }
                    }
                    6 => {
                        if action.value == Action::Attack {
                            atlas.index = 77;
                        } else {
                            atlas.index = 76;
                        }
                    }
                    7 => {
                        if action.value == Action::Attack {
                            atlas.index = 79;
                        } else {
                            atlas.index = 78;
                        }
                    }
                    _ => {}
                }
            }
            State::Dead => {
            }
            _ => {
                atlas.index = direction * 8;
            }
        }
    }
}
