use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use crate::input::component::{Selected, Selectable};

pub mod component;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (
                    draw_gizmo,
                    gizmo_config,
                    ));
    }
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
    ) {
    let unit_radius = 10.0;
    for n in 0..10 {
        commands.spawn((MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: unit_radius })),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz((n * 20) as f32, (n * 20) as f32, 0.0),
            ..default()
        },
        Selectable,
        component::Radius { value: unit_radius },
        ));
    }
}

pub fn draw_gizmo(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Selected>>
    ) {
    for transform in query.iter() {
        gizmos.circle_2d(transform.translation.xy(), 15.5, Color::GREEN);
    }
}

pub fn gizmo_config(
    mut config_store: ResMut<GizmoConfigStore>,
    ) {
        let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
        config.line_width = 1.0;
}
