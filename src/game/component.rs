use bevy::prelude::*;

#[derive(Component)]
pub struct Group {
    pub keycode: KeyCode,
    pub entities: Vec<Entity>,
}
