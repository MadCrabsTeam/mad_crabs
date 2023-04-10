use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{utils::remove_all_with, GameAssets, GlobalState};

use super::{castle::CastleWall, enemies::Enemy, East, GameState, North, Side, South, West};

pub mod area;
pub mod projectile;

const DAMAGE_TEXT_LIFESPAN: f32 = 1.0;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDamageEvent<North>>()
            .add_event::<EnemyDamageEvent<South>>()
            .add_event::<EnemyDamageEvent<West>>()
            .add_event::<EnemyDamageEvent<East>>()
            .add_event::<WallDamageEvent<North>>()
            .add_event::<WallDamageEvent<South>>()
            .add_event::<WallDamageEvent<West>>()
            .add_event::<WallDamageEvent<East>>()
            .add_plugin(area::AreaPlugin)
            .add_plugin(projectile::ProjectilePlugin)
            .add_systems(
                (
                    damage_enemy::<North>,
                    damage_enemy::<South>,
                    damage_enemy::<West>,
                    damage_enemy::<East>,
                    damage_wall::<North>,
                    damage_wall::<South>,
                    damage_wall::<West>,
                    damage_wall::<East>,
                    damage_text_update,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(
                remove_all_with::<DamageTextMarker>.in_schedule(OnExit(GlobalState::InGame)),
            );
    }
}

/// Event to damage enemy
pub struct EnemyDamageEvent<S: Side> {
    pub target: Entity,
    pub damage: i32,
    pub was_crit: bool,
    _phantom: PhantomData<S>,
}

impl<S: Side> EnemyDamageEvent<S> {
    pub fn new(target: Entity, damage: i32, was_crit: bool) -> Self {
        Self {
            target,
            damage,
            was_crit,
            _phantom: PhantomData,
        }
    }
}

/// Event to damage castle wall
pub struct WallDamageEvent<S: Side> {
    pub damage: i32,
    _phantom: PhantomData<S>,
}

impl<S: Side> WallDamageEvent<S> {
    pub fn new(damage: i32) -> Self {
        Self {
            damage,
            _phantom: PhantomData,
        }
    }
}

#[derive(Component)]
pub struct DamageTextMarker {
    lifespan: Timer,
}

impl Default for DamageTextMarker {
    fn default() -> Self {
        Self {
            lifespan: Timer::from_seconds(DAMAGE_TEXT_LIFESPAN, TimerMode::Once),
        }
    }
}

/// Damage enemies based on the side
fn damage_enemy<S: Side>(
    game_assets: Res<GameAssets>,
    mut commands: Commands,
    mut events: EventReader<EnemyDamageEvent<S>>,
    mut enemies: Query<(&Transform, &mut Enemy<S>)>,
) {
    for event in events.iter() {
        if let Ok((transform, mut enemy)) = enemies.get_mut(event.target) {
            enemy.health -= event.damage;

            let mut damage_text_transform = *transform;
            damage_text_transform.translation.y += 5.0;
            damage_text_transform.translation.z += 1.0;

            let (color, font_size) = if event.was_crit {
                (Color::ORANGE_RED, 50.0)
            } else {
                (Color::GRAY, 40.0)
            };

            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        format!("{}", event.damage),
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size,
                            color,
                        },
                    ),
                    transform: damage_text_transform,
                    ..default()
                },
                DamageTextMarker::default(),
            ));
        }
    }
}

/// Damage wall based on the side
fn damage_wall<S: Side>(
    // game_assets: Res<GameAssets>,
    // mut commands: Commands,
    mut events: EventReader<WallDamageEvent<S>>,
    mut wall: Query<(&Transform, &mut CastleWall<S>)>,
) {
    let (_, mut wall) = wall.single_mut();
    for event in events.iter() {
        wall.health -= event.damage;

        // let mut damage_text_transform = *transform;
        // damage_text_transform.translation.y += 5.0;
        // damage_text_transform.translation.z += 10.0;

        // commands.spawn((
        //     Text2dBundle {
        //         text: Text::from_section(
        //             format!("{}", event.damage),
        //             TextStyle {
        //                 font: game_assets.font.clone(),
        //                 font_size: 120.0,
        //                 color: Color::MIDNIGHT_BLUE,
        //             },
        //         ),
        //         transform: damage_text_transform,
        //         ..default()
        //     },
        //     DamageTextMarker::default(),
        // ));
    }
}

fn damage_text_update(
    time: Res<Time>,
    mut commands: Commands,
    mut damage_text: Query<(Entity, &mut DamageTextMarker)>,
) {
    for (entity, mut marker) in damage_text.iter_mut() {
        if marker.lifespan.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}
