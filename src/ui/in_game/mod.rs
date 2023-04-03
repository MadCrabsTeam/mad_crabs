use bevy::prelude::*;

use crate::{
    game::GameState,
    impl_into_state,
    utils::{set_state, IntoState},
};

mod hud;
mod level_up;
mod pause;

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
                set_state::<UiInGameState, { UiInGameState::Pause as u8 }>
                    .in_schedule(OnEnter(GameState::Paused)),
            )
            .add_system(in_game_key_input.in_set(OnUpdate(UiInGameState::InGame)))
            .add_plugin(hud::HUDPlugin)
            .add_plugin(level_up::LevelUpPlugin)
            .add_plugin(pause::PausePlugin);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
enum UiInGameState {
    #[default]
    Disabled,
    InGame,
    Pause,
}
impl_into_state!(UiInGameState);

fn in_game_key_input(keyboard: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keyboard.pressed(KeyCode::Escape) {
        game_state.set(GameState::Paused);
    }
}
