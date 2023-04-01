use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

const WALL_LENGTH: f32 = 100.0;
const WALL_THICKNESS: f32 = 10.0;

pub struct CastlePlugin;

impl Plugin for CastlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_system(remove_all_with::<CastleMarker>.in_schedule(OnEnter(GlobalState::InGame)))
            .add_system(
                remove_all_with::<CastleWallMarker>.in_schedule(OnEnter(GlobalState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct Castle {
    pub exp: u32,
}

#[derive(Component)]
pub struct CastleMarker;

#[derive(Bundle)]
pub struct CastleBundle {
    castle: Castle,
    marker: CastleMarker,
}

impl Default for CastleBundle {
    fn default() -> Self {
        Self {
            castle: Castle { exp: 0 },
            marker: CastleMarker,
        }
    }
}

#[derive(Component)]
pub struct CastleWall {
    pub health: i32,
}

#[derive(Component)]
pub struct CastleWallMarker;

#[derive(Bundle)]
pub struct CastleWallBundle {
    rigid_body: RigidBody,
    collider: Collider,
    marker: CastleWallMarker,
}

impl Default for CastleWallBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(WALL_LENGTH / 2.0, WALL_LENGTH / 2.0),
            marker: CastleWallMarker,
        }
    }
}

/// Sets up castle in the center of the map
/// with 4 walls
fn setup(
    mut commands: Commands,
    // TODO replace with sprites
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let castle_mesh = meshes.add(shape::Box::new(WALL_LENGTH, WALL_LENGTH, 0.0).into());
    let castle_material = materials.add(ColorMaterial::from(Color::BLUE));
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: castle_mesh.into(),
            material: castle_material,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .insert(CastleBundle::default());

    let horizontal_wall_mesh = meshes.add(shape::Box::new(WALL_LENGTH, WALL_THICKNESS, 0.0).into());
    let vertical_wall_mesh = meshes.add(shape::Box::new(WALL_THICKNESS, WALL_LENGTH, 0.0).into());
    let wall_material = materials.add(ColorMaterial::from(Color::DARK_GRAY));

    // Walls have z of 1.0 to fix z fighting
    // North
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: horizontal_wall_mesh.clone().into(),
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 50.0, 1.0)),
            ..default()
        })
        .insert(Collider::cuboid(WALL_LENGTH / 2.0, WALL_THICKNESS / 2.0));
    // South
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: horizontal_wall_mesh.into(),
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            ..default()
        })
        .insert(Collider::cuboid(WALL_LENGTH / 2.0, WALL_THICKNESS / 2.0));
    // // West
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: vertical_wall_mesh.clone().into(),
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 1.0)),
            ..default()
        })
        .insert(Collider::cuboid(WALL_THICKNESS / 2.0, WALL_LENGTH / 2.0));
    // // East
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: vertical_wall_mesh.into(),
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(50.0, 0.0, 1.0)),
            ..default()
        })
        .insert(Collider::cuboid(WALL_THICKNESS / 2.0, WALL_LENGTH / 2.0));
}