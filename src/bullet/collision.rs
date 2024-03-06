use bevy::prelude::*;

use super::component::{Bullet, Damage};
use crate::unit::component;

pub fn collision (
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &component::Radius, &Damage), With<Bullet>>,
    mut unit_query: Query<(&Transform, &component::Radius, &mut component::Health), With<component::Unit>>,
    ) {
    for (bullet, bullet_transform, bullet_radius, damage) in bullet_query.iter() {
        for (unit_transform, unit_radius, mut health) in unit_query.iter_mut() {
            let distance = bullet_transform.translation.xy().distance(unit_transform.translation.xy());
            if distance < bullet_radius.value + unit_radius.value {
                health.current -= damage.value;
                commands.entity(bullet).despawn();
            }
        }
    }
}
