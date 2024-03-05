use bevy::prelude::*;

use super::component;
use crate::input::{component::{Selectable, Selected}, Reverse, Repeat};

const GHOST_COLOR: Color = Color::rgba(0.5, 0.5, 1.0, 0.3);
const ENEMY_COLOR: Color = Color::RED;
const DEFAULT_COLOR: Color = Color::WHITE;

#[derive(Clone)]
pub struct Snapshot {
    pub atlas_index: usize,
    pub action: super::action::Action,
    pub state: super::State,
    pub timestamp: f32,
    pub position: Vec3,
    pub facing: f32,
    pub direction: f32,
}

pub fn track_history(
    mut queue: Query<(&mut component::History, &TextureAtlas, &Transform, &component::Facing, &component::CurrentState, &component::CurrentAction), (With<component::Unit>, Without<component::Repeat>, Without<component::Reverse>)>,
    time: Res<Time>,
    ) {
    for (mut history, atlas, transform, facing, state, action) in queue.iter_mut() {
        let z = transform.rotation.to_euler(EulerRot::XYZ).2;
        history.snapshots.push_back(Snapshot {
            atlas_index: atlas.index,
            action: action.value,
            state: state.value,
            timestamp: time.elapsed_seconds(),
            position: transform.translation,
            facing: facing.value,
            direction: z,
        });
    }
}

pub fn repeat_history(
    mut commands: Commands,
    mut queue: Query<(Entity, &mut Sprite, &mut Transform, &mut component::Facing, &mut component::History, &mut TextureAtlas, &mut component::CurrentState, &mut component::CurrentAction, Option<&component::Repeat>, Option<&component::Reverse>, Option<&component::Enemy>), (With<component::Unit>, With<component::Ghost>)>,
    ) {
    for (entity, mut sprite, mut transform, mut facing, mut history, mut atlas, mut state, mut action, opt_repeat, opt_reverse, opt_enemy) in queue.iter_mut() {
        if let Some(_) = opt_repeat {
            if let Some(snapshot) = history.snapshots.pop_front() {
                let mut historical_transform = Transform::from_translation(snapshot.position);
                historical_transform.rotate_z(snapshot.direction);
                *transform = historical_transform;
                state.value = snapshot.state;
                action.value = snapshot.action;
                atlas.index = snapshot.atlas_index;
                facing.value = snapshot.facing;
                if action.value == super::action::Action::Attack {
                    warn!("Repeat Attack");
                }
            } else {
                if let Some(_) = opt_enemy {
                    sprite.color = ENEMY_COLOR;
                } else {
                    sprite.color = DEFAULT_COLOR;
                    commands.entity(entity).insert(Selectable);
                }
                commands.entity(entity).remove::<component::Ghost>();
                commands.entity(entity).remove::<component::Repeat>();
            }
        }
        if let Some(_) = opt_reverse {
            if let Some(snapshot) = history.snapshots.pop_back() {
                let mut historical_transform = Transform::from_translation(snapshot.position);
                historical_transform.rotate_z(snapshot.direction);
                *transform = historical_transform;
                state.value = snapshot.state;
                action.value = snapshot.action;
                atlas.index = snapshot.atlas_index;
                facing.value = snapshot.facing;
                if action.value == super::action::Action::Attack {
                    warn!("Reverse Attack");
                }
            } else {
                if let Some(_) = opt_enemy {
                    sprite.color = ENEMY_COLOR;
                } else {
                    sprite.color = DEFAULT_COLOR;
                    commands.entity(entity).insert(Selectable);
                }
                commands.entity(entity).remove::<component::Ghost>();
                commands.entity(entity).remove::<component::Reverse>();
            }
        }
    }
}

pub fn start_reverse(
    mut commands: Commands,
    mut reverse_reader: EventReader<Reverse>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<(Entity, &component::Unit, &mut Sprite, &mut component::Target, &component::History, &component::Radius, &component::TurnRate, &component::MoveSpeed, &component::Facing, &component::CurrentState, &component::CurrentAction, &component::Attack, &component::AnimationIndices, &component::AnimationTimer, Option<&component::Enemy>)>,
    time: Res<Time>,
    ) {
    for event in reverse_reader.read() {
        if !event.1 {
            if let Ok((entity, unit, mut sprite, mut target, history, _, _, _, _, _, _, _, _, _, opt_enemy)) = query.get_mut(event.0) {
                sprite.color = GHOST_COLOR;
                if let Some(first_snapshot) = history.snapshots.front() {
                    target.x = first_snapshot.position.x;
                    target.y = first_snapshot.position.y;
                    if let None = opt_enemy {
                        commands.entity(entity).remove::<Selected>();
                        commands.entity(entity).remove::<Selectable>();
                    }
                    commands.entity(entity).insert(component::Ghost);
                    commands.entity(entity).insert(component::Reverse { timestamp: time.elapsed_seconds() });
                }
            }
        } else {
            if let Ok((_, unit, _, _, history, radius, turn_rate, move_speed, facing, state, action, attack, anim_indices, anim_timer, opt_enemy)) = query.get(event.0) {
                if let Some(last_snapshot) = history.snapshots.back() {
                    if let Some(first_snapshot) = history.snapshots.front() {
                        let texture = asset_server.load::<Image>("marine.png");
                        let layout = TextureAtlasLayout::from_grid(Vec2::new(100.0, 100.0), 8, 8, None, None);
                        let texture_atlas_layout = texture_atlas_layouts.add(layout);
                        let spawn_transform = Transform::from_xyz(last_snapshot.position.x, last_snapshot.position.y, 0.0);
                        let parent = commands.spawn((SpriteSheetBundle {
                            sprite: Sprite {
                                color: GHOST_COLOR,
                                ..default()
                            },
                            texture,
                            atlas: TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 0,
                            },
                            transform: spawn_transform,
                            ..default()
                        },
                        component::Unit { owner: unit.owner },
                        ))
                        .insert((
                        component::Ghost,
                        component::Radius { value: radius.value },
                        component::Velocity { x: 0.0, y: 0.0 },
                        component::MoveSpeed { value: move_speed.value },
                        component::Facing { value: facing.value },
                        component::TurnRate { value: turn_rate.value },
                        component::Target { entity: None, x: first_snapshot.position.x, y: first_snapshot.position.y},
                        component::Attack { range: attack.range, timer: attack.timer.clone() },
                        component::CurrentAction { value: action.value },
                        component::CurrentState { value: state.value },
                        component::History { snapshots: history.snapshots.clone() },
                        component::AnimationIndices { current: anim_indices.current, first: anim_indices.first, last: anim_indices.last },
                        component::AnimationTimer { timer: anim_timer.timer.clone() },
                        component::Reverse { timestamp: time.elapsed_seconds() },
                        )).id();

                        if let Some(_) = opt_enemy {
                            commands.entity(parent).insert(component::Enemy);
                        }

                        let child_texture = asset_server.load::<Image>("selection_circle.png");
                        let child = commands.spawn(
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::WHITE,
                                    ..default()
                                },
                                texture: child_texture,
                                transform: Transform::from_xyz(0.0, -25.0, -100.0),
                                visibility: Visibility::Hidden,
                                ..default()
                            }).id();

                        commands.entity(parent).add_child(child);
                    }
                }
            }
        }
    }
}

pub fn start_repeat(
    mut commands: Commands,
    mut repeat_reader: EventReader<Repeat>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<(Entity, &component::Unit, &mut Sprite, &mut component::Target, &component::History, &component::Radius, &component::TurnRate, &component::MoveSpeed, &component::Facing, &component::CurrentState, &component::CurrentAction, &component::Attack, &component::AnimationIndices, &component::AnimationTimer, Option<&component::Enemy>)>,
    time: Res<Time>,
    ) {
    for event in repeat_reader.read() {
        if !event.1 {
            if let Ok((entity, unit, mut sprite, mut target, history, _, _, _, _, _, _, _, _, _, opt_enemy)) = query.get_mut(event.0) {
                sprite.color = GHOST_COLOR;
                if let Some(last_snapshot) = history.snapshots.back() {
                target.x = last_snapshot.position.x;
                target.y = last_snapshot.position.y;
                if let None = opt_enemy {
                    commands.entity(entity).remove::<Selected>();
                    commands.entity(entity).remove::<Selectable>();
                }
                commands.entity(entity).insert(component::Ghost);
                commands.entity(entity).insert(component::Repeat{timestamp: time.elapsed_seconds()});
                }
            }
        } else {
            if let Ok((_, unit, _, _, history, radius, turn_rate, move_speed, facing, state, action, attack, anim_indices, anim_timer, opt_enemy)) = query.get(event.0) {
                if let Some(first_snapshot) = history.snapshots.front() {
                    if let Some(last_snapshot) = history.snapshots.back() {
                        let texture = asset_server.load::<Image>("marine.png");
                        let layout = TextureAtlasLayout::from_grid(Vec2::new(100.0, 100.0), 8, 8, None, None);
                        let texture_atlas_layout = texture_atlas_layouts.add(layout);
                        let spawn_transform = Transform::from_xyz(first_snapshot.position.x, first_snapshot.position.y, 0.0);
                        let parent = commands.spawn((SpriteSheetBundle {
                            sprite: Sprite {
                                color: GHOST_COLOR,
                                ..default()
                            },
                            texture,
                            atlas: TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 0,
                            },
                            transform: spawn_transform,
                            ..default()
                        },
                        component::Unit { owner: unit.owner },
                        ))
                        .insert((
                        component::Ghost,
                        component::Radius { value: radius.value },
                        component::Velocity { x: 0.0, y: 0.0 },
                        component::MoveSpeed { value: move_speed.value },
                        component::Facing { value: facing.value },
                        component::TurnRate { value: turn_rate.value },
                        component::Target { entity: None, x: last_snapshot.position.x, y: last_snapshot.position.y},
                        component::Attack { range: attack.range, timer: attack.timer.clone() },
                        component::CurrentAction { value: action.value },
                        component::CurrentState { value: state.value },
                        component::History { snapshots: history.snapshots.clone() },
                        component::AnimationIndices { current: anim_indices.current, first: anim_indices.first, last: anim_indices.last },
                        component::AnimationTimer { timer: anim_timer.timer.clone() },
                        component::Repeat { timestamp: time.elapsed_seconds() },
                        )).id();

                        if let Some(_) = opt_enemy {
                            commands.entity(parent).insert(component::Enemy);
                        }

                        let child_texture = asset_server.load::<Image>("selection_circle.png");
                        let child = commands.spawn(
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::WHITE,
                                    ..default()
                                },
                                texture: child_texture,
                                transform: Transform::from_xyz(0.0, -25.0, -100.0),
                                visibility: Visibility::Hidden,
                                ..default()
                            }).id();

                        commands.entity(parent).add_child(child);
                    }
                }
            }
        }
    }
}
