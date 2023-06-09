use bevy::prelude::*;

use crate::{
    game::{East, GameState, North, South, West},
    impl_into_state,
    utils::{set_state, IntoState},
};

mod game_over;
mod hud;
mod level_up;
mod pause;
mod side_stats;

pub struct UiInGamePlugin;

impl Plugin for UiInGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UiInGameState>()
            .add_system(
                set_state::<UiInGameState, { UiInGameState::InGame as u8 }>
                    .in_schedule(OnEnter(GameState::InGame)),
            )
            .add_system(
                set_state::<UiInGameState, { UiInGameState::Disabled as u8 }>
                    .in_schedule(OnEnter(GameState::NotInGame)),
            )
            .add_system(
                set_state::<UiInGameState, { UiInGameState::LevelUp as u8 }>
                    .in_schedule(OnEnter(GameState::LevelUp)),
            )
            .add_system(
                set_state::<UiInGameState, { UiInGameState::Pause as u8 }>
                    .in_schedule(OnEnter(GameState::Paused)),
            )
            .add_system(
                set_state::<UiInGameState, { UiInGameState::GameOver as u8 }>
                    .in_schedule(OnEnter(GameState::GameOver)),
            )
            .add_system(
                set_state::<UiInGameState, { UiInGameState::StatsNorth as u8 }>
                    .in_schedule(OnEnter(GameState::StatsNorth)),
            )
            .add_system(
                set_state::<UiInGameState, { UiInGameState::StatsSouth as u8 }>
                    .in_schedule(OnEnter(GameState::StatsSouth)),
            )
            .add_system(
                set_state::<UiInGameState, { UiInGameState::StatsWest as u8 }>
                    .in_schedule(OnEnter(GameState::StatsWest)),
            )
            .add_system(
                set_state::<UiInGameState, { UiInGameState::StatsEast as u8 }>
                    .in_schedule(OnEnter(GameState::StatsEast)),
            )
            .add_plugin(hud::HUDPlugin)
            .add_plugin(level_up::LevelUpPlugin)
            .add_plugin(pause::PausePlugin)
            .add_plugin(game_over::GameOverPlugin)
            .add_plugin(side_stats::StatsPlugin::<North>::default())
            .add_plugin(side_stats::StatsPlugin::<South>::default())
            .add_plugin(side_stats::StatsPlugin::<West>::default())
            .add_plugin(side_stats::StatsPlugin::<East>::default());
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
enum UiInGameState {
    #[default]
    Disabled,
    InGame,
    Pause,
    GameOver,
    LevelUp,
    StatsNorth,
    StatsSouth,
    StatsWest,
    StatsEast,
}
impl_into_state!(UiInGameState);
