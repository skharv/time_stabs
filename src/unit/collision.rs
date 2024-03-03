use bevy::prelude::*;

use super::component;

pub fn collision(
    mut query: Query<(&mut Transform, &mut component::Velocity, &component::Radius), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut unit1, mut unit2]) = combinations.fetch_next() {
        let distance = unit1.0.translation.distance(unit2.0.translation).max(1.0);
        let combined_radius = unit1.2.value + unit2.2.value;
        if distance < combined_radius {
            let normal = (unit1.0.translation - unit2.0.translation).max(Vec3::ONE).normalize();
            let unit1_percent = unit1.2.value / combined_radius * 100.0;
            let unit2_percent = unit2.2.value / combined_radius * 100.0;
            unit1.1.x += normal.x * unit1_percent;
            unit1.1.y += normal.y * unit1_percent;
            unit2.1.x -= normal.x * unit2_percent;
            unit2.1.y -= normal.y * unit2_percent;
        }
    }
}
