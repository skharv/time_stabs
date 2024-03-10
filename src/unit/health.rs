use bevy::prelude::*;
use rand::Rng;
use super::{component, State};

pub const HEALTH_BAR_HEIGHT: f32 = 6.0;
pub const HEALTH_BAR_WIDTH: f32 = 50.0;
pub const HEALTH_BAR_BORDER: f32 = 2.0;

pub fn health(
    mut commands: Commands,
    mut dying_query: Query<(Entity, &mut Transform, &Children, &component::Health, &mut component::CurrentState, &mut TextureAtlas)>,
    mut target_query: Query<&mut component::Target, With<component::Unit>>,
    ) {
    for (entity, mut transform, children, health, mut state, mut atlas) in dying_query.iter_mut() {
        if health.current <= 0 {
            let mut rng = rand::thread_rng();
            for mut target in target_query.iter_mut() {
                if target.entity == Some(entity) {
                    target.entity = None;
                }
            }
            if state.value != State::Dead {
                state.value = State::Dead;
                transform.translation.z -= 200.0;
                atlas.index = rng.gen_range(80..83);
                for &child in children.iter() {
                    commands.entity(child).despawn_recursive();
                }
                commands.entity(entity).insert(component::Dead);
            }
        }
    }
}

pub fn health_ui(
    mut query: Query<(&component::Health, &Children), With<component::Unit>>,
    mut bar_query: Query<(&mut Transform, &mut Visibility), (With<component::HealthBarAmountUi>, Without<component::HealthBarUi>)>,
    mut border_query: Query<(&mut Transform, &mut Visibility), (With<component::HealthBarUi>, Without<component::HealthBarAmountUi>)>,
    ) {
    for (health, children) in query.iter_mut() {
        if health.current == health.max {
            for &child in children.iter() {
                if let Ok((_, mut visibility)) = bar_query.get_mut(child) {
                    *visibility = Visibility::Hidden;
                }
                if let Ok((_, mut visibility)) = border_query.get_mut(child) {
                    *visibility = Visibility::Hidden;
                }
            }
        } else {
            for &child in children.iter() {
                if let Ok((_, mut visibility)) = bar_query.get_mut(child) {
                    *visibility = Visibility::Visible;
                }
                if let Ok((_, mut visibility)) = border_query.get_mut(child) {
                    *visibility = Visibility::Visible;
                }
            }
        }

        for &child in children.iter() {
            if let Ok((mut transform, _)) = bar_query.get_mut(child) {
                let scale = health.current as f32 / health.max as f32;
                transform.scale.x = scale;
                transform.translation.x = -(HEALTH_BAR_WIDTH / 2.0) + (HEALTH_BAR_WIDTH * scale / 2.0);
            }
        }
    }
}
