use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub mod map;
use crate map::MapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    let square = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0)));
    let circle = Mesh2dHandle(meshes.add(Circle { radius: 25.0 }));
    for (row_index, row) in map_info.iter().enumerate() {
        for (col_index, node) in row.iter().enumerate() {
            let color = match node.controller {
                Nation::Lebanon => Color::rgb(0.0, 1.0, 0.0),   // Green
                Nation::Iraq => Color::rgb(1.0, 0.0, 0.0),      // Red
                Nation::Syria => Color::rgb(0.0, 0.0, 1.0),     // Blue
                Nation::Jordan => Color::rgb(1.0, 1.0, 0.0),    // Yellow
                Nation::Egypt => Color::rgb(1.0, 0.0, 1.0),     // Magenta
                _ => Color::rgb(1.0, 1.0, 1.0),                // Default to white for uncontrolled
            };

            let mesh_handle = match node.node_type {
                NodeType::City => square,
                NodeType::Village => circle,
                _ => continue,
            };

            commands.spawn(MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: materials.add(color),
                transform: node.transform.clone(),
                ..Default::default()
            });
        }
    }
}