use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

use super::{
    East, EnemyBundle, EnemyMarker, EnemySprites, Goblin, North, Side, South, SpearGoblin, West,
};

const DEFAULT_ENEMY_SIZE: f32 = 16.0;

const DEFAULT_ENEMY_SPAWN_NUMBER: u32 = 1;
const DEFAULT_ENEMY_SPAWN_RADIUS: f32 = 200.0;
const DEFAULT_ENEMY_SPAWN_RATE: f32 = 5.0;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    enemy_spawn::<North>,
                    enemy_spawn::<South>,
                    enemy_spawn::<West>,
                    enemy_spawn::<East>,
                )
                    .in_set(OnUpdate(GlobalState::InGame)),
            )
            .add_system(remove_all_with::<EnemyMarker>.in_schedule(OnEnter(GlobalState::MainMenu)));
    }
}

#[derive(Debug, Component)]
pub struct EnemySpawnBuffs {
    pub health: f32,
    pub speed: f32,
    pub exp: f32,
    pub damage: f32,
    pub attack_speed: f32,
}

impl Default for EnemySpawnBuffs {
    fn default() -> Self {
        Self {
            health: 1.0,
            speed: 1.0,
            exp: 1.0,
            damage: 1.0,
            attack_speed: 1.0,
        }
    }
}

#[derive(Debug, Component)]
pub struct EnemySpawn<S: Side> {
    pub number: u32,
    pub radius: f32,
    pub timer: Timer,
    pub side: S,
}

impl<S: Side> Default for EnemySpawn<S> {
    fn default() -> Self {
        Self {
            number: DEFAULT_ENEMY_SPAWN_NUMBER,
            radius: DEFAULT_ENEMY_SPAWN_RADIUS,
            timer: Timer::from_seconds(DEFAULT_ENEMY_SPAWN_RATE, TimerMode::Repeating),
            side: S::default(),
        }
    }
}

#[derive(Default, Bundle)]
pub struct EnemySpawnBundle<S: Side> {
    spawn: EnemySpawn<S>,
    buffs: EnemySpawnBuffs,
}

/// Sets up 4 spawns at each side of the screen
fn setup(
    mut commands: Commands,
    // TODO replace with sprites
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let spawn_mesh = meshes.add(shape::Circle::new(15.0).into());
    let spawn_material = materials.add(ColorMaterial::from(Color::ORANGE));

    // North
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 500.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawnBundle::<North>::default());
    // South
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -500.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawnBundle::<South>::default());
    // West
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(-500.0, 0.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawnBundle::<West>::default());
    // East
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.into(),
            material: spawn_material,
            transform: Transform::from_translation(Vec3::new(500.0, 0.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawnBundle::<East>::default());
}

/// Spawns enemies in a circle arond the spawn point equally spread
/// on a circle
fn enemy_spawn<S: Side>(
    time: Res<Time>,
    enemy_sprites: Res<EnemySprites>,
    mut commands: Commands,
    mut spawns: Query<(&Transform, &EnemySpawnBuffs, &mut EnemySpawn<S>)>,
) {
    for (transform, buffs, mut spawn) in spawns.iter_mut() {
        if !spawn.timer.tick(time.delta()).finished() {
            continue;
        }

        for n in 0..spawn.number {
            let position = transform.translation
                + Quat::from_rotation_z(
                    (2.0 * std::f32::consts::PI / spawn.number as f32) * n as f32,
                )
                .mul_vec3(Vec3::Y * spawn.radius);

            // Choose enemy at random for now
            if random() {
                commands.spawn(EnemyBundle::<S, Goblin>::new(
                    DEFAULT_ENEMY_SIZE,
                    enemy_sprites.goblin.clone(),
                    position,
                    buffs,
                ));
            } else {
                commands.spawn(EnemyBundle::<S, SpearGoblin>::new(
                    DEFAULT_ENEMY_SIZE,
                    enemy_sprites.spear_goblin.clone(),
                    position,
                    buffs,
                ));
            }
        }
    }
}