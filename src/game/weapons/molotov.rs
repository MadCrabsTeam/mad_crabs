use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

use crate::{
    game::{
        damage::area::{DamageArea, DamageAreaBundle},
        East, GameState, North, Side, South, West, castle::CastleWall,
    },
    utils::remove_all_with,
    GameAssets, GameSettings, GlobalState,
};

use super::{GlobalWeaponBuffs, WeaponsAssets};

const DEFAULT_AREA_SIZE: f32 = 20.0;
const DEFAULT_AREA_DAMAGE: i32 = 10;
const DEFAULT_AREA_ATTACK_SPEED: f32 = 0.5;
const DEFAULT_AREA_LIFESPAN: f32 = 2.0;

const DEFAULT_MOLOTOV_MIN_RANGE: f32 = 30.0;
const DEFAULT_MOLOTOV_RANGE: f32 = 300.0;
const DEFAULT_MOLOTOV_ATTACK_SPEED: f32 = 0.3;
const DEFAULT_MOLOTOV_BOTTLE_IN_FLIGHT_TIME: f32 = 2.0;
const DEFAULT_MOLOTOV_BOTTLE_IN_FLIGHT_ROTATION: f32 = std::f32::consts::PI * 5.0;

// const DEFAULT_MOLOTOV_SPAWN_OFFSET: f32 = 30.0;

const MOLOTOV_SFX_MULTIPLIER: f64 = 0.2;

pub struct MolotovPlugin;

impl Plugin for MolotovPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    molotov_attack::<North>,
                    molotov_attack::<South>,
                    molotov_attack::<West>,
                    molotov_attack::<East>,
                    molotov_bottle_update::<North>,
                    molotov_bottle_update::<South>,
                    molotov_bottle_update::<West>,
                    molotov_bottle_update::<East>,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(remove_all_with::<MolotovMarker>.in_schedule(OnExit(GlobalState::InGame)));
    }
}

#[derive(Component)]
pub struct MolotovMarker;

#[derive(Default, Resource)]
pub struct MolotovBuffs<S: Side> {
    pub damage: f32,
    pub damage_flat: i32,
    pub crit_damage: f32,
    pub crit_chance: f32,
    pub area_size: f32,
    pub attack_speed: f32,
    pub area_attack_speed: f32,
    pub area_lifespan: f32,
    _phatom: PhantomData<S>,
}

#[derive(Component)]
pub struct Molotov<S: Side> {
    damage: i32,
    range: f32,
    area_size: f32,
    area_attack_speed: f32,
    area_lifespan: f32,
    attack_timer: Timer,
    _phantom: PhantomData<S>,
}

impl<S: Side> std::fmt::Display for Molotov<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("damage {}\n", self.damage))?;
        f.write_fmt(format_args!("range {:.1}\n", self.range))?;
        f.write_fmt(format_args!("area size {:.1}\n", self.area_size))?;
        f.write_fmt(format_args!(
            "area attack speed {:.1}/s\n",
            self.area_attack_speed
        ))?;
        f.write_fmt(format_args!("area lifespan {:.1}s\n", self.area_lifespan))?;
        f.write_fmt(format_args!(
            "attack speed {:.1}/s\n",
            self.attack_timer.duration().as_secs_f32()
        ))?;
        Ok(())
    }
}

impl<S: Side> Default for Molotov<S> {
    fn default() -> Self {
        Self {
            damage: DEFAULT_AREA_DAMAGE,
            range: DEFAULT_MOLOTOV_RANGE,
            area_size: DEFAULT_AREA_SIZE,
            area_attack_speed: 1.0 / DEFAULT_AREA_ATTACK_SPEED,
            area_lifespan: DEFAULT_AREA_LIFESPAN,
            attack_timer: Timer::from_seconds(DEFAULT_MOLOTOV_ATTACK_SPEED, TimerMode::Repeating),
            _phantom: PhantomData,
        }
    }
}

#[derive(Component)]
pub struct MolotovBottle<S: Side> {
    area: DamageArea<S>,
    rotation: f32,
    initial_position: Vec3,
    target_position: Vec3,
}

#[derive(Bundle)]
pub struct MolotovBottleBundle<S: Side> {
    #[bundle]
    sprite: SpriteBundle,
    bottle: MolotovBottle<S>,
    marker: MolotovMarker,
}

impl<S: Side> MolotovBottleBundle<S> {
    pub fn new(
        texture: Handle<Image>,
        area_size: f32,
        damage: i32,
        crit_damage: i32,
        crit_chance: f32,
        attack_speed: f32,
        lifespan: f32,
        area_position: Vec3,
        initial_position: Vec3,
    ) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    // Flip on East and North
                    flip_y: S::DIRECTION.x < 0.0 || S::DIRECTION.y > 0.0,
                    ..default()
                },
                texture,
                transform: Transform::from_translation(initial_position),
                ..default()
            },
            bottle: MolotovBottle {
                area: DamageArea::new(
                    area_size,
                    damage,
                    crit_damage,
                    crit_chance,
                    attack_speed,
                    lifespan,
                ),
                rotation: 0.0,
                initial_position,
                target_position: area_position,
            },
            marker: MolotovMarker,
        }
    }
}

impl<S: Side> Molotov<S> {
    pub fn with_buffs(
        self,
        molotov_buffs: &MolotovBuffs<S>,
        global_weapons_buffs: &GlobalWeaponBuffs,
    ) -> Self {
        Self {
            damage: ((self.damage + molotov_buffs.damage_flat + global_weapons_buffs.damage_flat)
                as f32
                * (1.0 + molotov_buffs.damage + global_weapons_buffs.damage))
                as i32,
            range: self.range,
            area_size: self.area_size * (1.0 + molotov_buffs.area_size),
            area_attack_speed: self.area_attack_speed * (1.0 + molotov_buffs.area_attack_speed),
            area_lifespan: self.area_lifespan * (1.0 + molotov_buffs.area_lifespan),
            attack_timer: Timer::from_seconds(
                DEFAULT_MOLOTOV_ATTACK_SPEED * (1.0 + molotov_buffs.attack_speed),
                TimerMode::Repeating,
            ),
            _phantom: PhantomData,
        }
    }
}

#[derive(Bundle)]
pub struct MolotovBundle<S: Side> {
    molotov: Molotov<S>,
    marker: MolotovMarker,
}

impl<S: Side> Default for MolotovBundle<S> {
    fn default() -> Self {
        Self {
            molotov: Default::default(),
            marker: MolotovMarker,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(MolotovBuffs::<North>::default());
    commands.insert_resource(MolotovBuffs::<South>::default());
    commands.insert_resource(MolotovBuffs::<West>::default());
    commands.insert_resource(MolotovBuffs::<East>::default());
}

fn molotov_attack<S: Side>(
    time: Res<Time>,
    weapon_assets: Res<WeaponsAssets>,
    molotov_buffs: Res<MolotovBuffs<S>>,
    global_weapons_buffs: Res<GlobalWeaponBuffs>,
    mut commands: Commands,
    mut molotovs: Query<(&Transform, &CastleWall<S>, &mut Molotov<S>)>,
) {
    for (transform, wall, mut molotov) in molotovs.iter_mut() {
        if !molotov.attack_timer.tick(time.delta()).finished() {
            continue;
        }

        molotov.attack_timer = Timer::from_seconds(
            1.0 / DEFAULT_MOLOTOV_ATTACK_SPEED * (1.0 + molotov_buffs.attack_speed),
            TimerMode::Repeating,
        );

        let mut rng = rand::thread_rng();
        // each side is 60 degrees in size.
        // S::direction gives a line directly at the center of the side
        let angle = rng.gen_range(-30.0..30.0);
        let distance = rng.gen_range(DEFAULT_MOLOTOV_MIN_RANGE..molotov.range);

        // convert angle to radians
        let direction = Vec2::from_angle(angle / 360.0 * std::f32::consts::PI).rotate(S::DIRECTION);

        let mut initial_position = transform.translation;
        initial_position += (direction * wall.half_thickness).extend(0.0);

        let mut area_position = transform.translation;
        area_position += (direction * (distance + wall.half_thickness)).extend(0.0);

        let damage =
            ((molotov.damage + molotov_buffs.damage_flat + global_weapons_buffs.damage_flat) as f32
                * (1.0 + molotov_buffs.damage + global_weapons_buffs.damage)) as i32;
        let area_size = molotov.area_size * (1.0 + molotov_buffs.area_size);
        let area_attack_speed = molotov.area_attack_speed * (1.0 + molotov_buffs.area_attack_speed);
        let area_lifespan = DEFAULT_AREA_LIFESPAN * (1.0 + molotov_buffs.area_lifespan);
        let crit_chance = molotov_buffs.crit_chance + global_weapons_buffs.crit_chance;
        let crit_damage = (molotov.damage as f32
            * (1.0 + molotov_buffs.crit_damage + global_weapons_buffs.crit_damage))
            as i32;

        commands.spawn(MolotovBottleBundle::<S>::new(
            weapon_assets.molotov.clone(),
            area_size,
            damage,
            crit_damage,
            crit_chance,
            area_attack_speed,
            area_lifespan,
            area_position,
            initial_position,
        ));
    }
}

fn molotov_bottle_update<S: Side>(
    time: Res<Time>,
    audio: Res<Audio>,
    game_assets: Res<GameAssets>,
    game_settings: Res<GameSettings>,
    weapon_assets: Res<WeaponsAssets>,
    mut commands: Commands,
    mut bottles: Query<(Entity, &mut MolotovBottle<S>, &mut Transform)>,
) {
    for (entity, mut bottle, mut transform) in bottles.iter_mut() {
        let direction = bottle.target_position - bottle.initial_position;

        let distance = direction.length();
        let speed = distance / DEFAULT_MOLOTOV_BOTTLE_IN_FLIGHT_TIME;
        transform.translation += direction.normalize() * speed * time.delta().as_secs_f32();

        let progression = (bottle.initial_position - transform.translation).length() / distance;
        let rotation = progression * DEFAULT_MOLOTOV_BOTTLE_IN_FLIGHT_ROTATION;
        let rotation_delta = rotation - bottle.rotation;
        bottle.rotation = rotation;

        // For East and South rotate in opposide direction
        if S::DIRECTION.x > 0.0 || S::DIRECTION.y < 0.0 {
            transform.rotate_z(-rotation_delta);
        } else {
            transform.rotate_z(rotation_delta);
        }

        if 1.0 <= progression {
            commands.entity(entity).despawn();

            commands.spawn(DamageAreaBundle::<S>::new(
                weapon_assets.fire.clone(),
                bottle.target_position,
                bottle.area.clone(),
            ));

            audio
                .play(game_assets.explosion.clone())
                .with_volume(game_settings.sound_volume * MOLOTOV_SFX_MULTIPLIER);
        }
    }
}
