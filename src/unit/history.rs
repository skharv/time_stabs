use bevy::prelude::*;

use super::component;
use crate::input::component::Selectable;

pub struct Snapshot {
    pub timestamp: f32,
    pub position: Vec2,
}

pub fn track_history(
    mut queue: Query<(&mut component::History, &Transform), (With<component::Unit>, Without<component::Repeat>)>,
    time: Res<Time>,
    ) {
    for (mut history, transform) in queue.iter_mut() {
        history.snapshots.push_back(Snapshot {
            timestamp: time.elapsed_seconds(),
            position: transform.translation.xy(),
        });
    }
}

pub fn repeat_history(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut queue: Query<(Entity, &Handle<ColorMaterial>, &mut Transform, &mut component::History, Option<&component::Repeat>, Option<&component::Reverse>), (With<component::Unit>, With<component::Ghost>)>,
    ) {
    for (entity, handle, mut transform, mut history, &opt_repeat, &opt_reverse) in queue.iter_mut() {
        if let Some(_) = opt_repeat {
            if let Some(snapshot) = history.snapshots.pop_front() {
                transform.translation = Vec3::new(snapshot.position.x, snapshot.position.y, 0.0);
            } else {
                if let Some(material) = materials.get_mut(handle) {
                    material.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
                }
                commands.entity(entity).remove::<component::Ghost>();
                commands.entity(entity).remove::<component::Repeat>();
                commands.entity(entity).insert(Selectable);
            }
        }
    }
}


