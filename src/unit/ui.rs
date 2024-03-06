use bevy::prelude::*;

use super::component::Health;

pub fn health_bar_ui(
    mut commands: Commands,
    query: Query<(Entity, &Health, &Transform)>,
    asset_server: Res<AssetServer>,
    cameras: Query<(&Transform, &Camera)>,
    windows: Query<&Window>,
    ) {
    let window = windows.single();
    let (camera_transform, camera) = cameras.single();
}
