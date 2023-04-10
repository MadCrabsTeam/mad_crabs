#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution},
};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::{AudioPlugin, AudioSource};
use bevy_rapier2d::prelude::*;

mod game;
mod ui;
mod utils;

use utils::IntoState;

const GAME_NAME: &str = "Mind your sides";

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb_u8(24, 20, 37)))
        .add_state::<GlobalState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: GAME_NAME.to_string(),
                        mode: WindowMode::Windowed,
                        resolution: WindowResolution::new(1280.0, 720.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_loading_state(
            LoadingState::new(GlobalState::AssetLoading)
                .continue_to_state(GlobalState::Initialization),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GlobalState::AssetLoading)
        .add_plugin(AudioPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(ui::UiPlugin)
        .add_system(setup.in_set(OnUpdate(GlobalState::Initialization)));
    app.run();
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
pub enum GlobalState {
    #[default]
    AssetLoading,
    /// Init resources that depend on assets
    /// i.e UI resources
    Initialization,
    MainMenu,
    InGame,
}
impl_into_state!(GlobalState);

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "fonts/ae-systematic-tt-brk.ae-systematic-tt-brk.ttf")]
    font: Handle<Font>,
    #[asset(path = "sfx/background.wav")]
    background: Handle<AudioSource>,
    #[asset(path = "sfx/main_menu.wav")]
    main_menu: Handle<AudioSource>,
    #[asset(path = "sfx/crossbow_shoot.wav")]
    crossbow_shoot: Handle<AudioSource>,
    #[asset(path = "sfx/explosion.wav")]
    explosion: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct GameSettings {
    window_mode: WindowMode,
    sound_volume: f64,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            window_mode: WindowMode::Windowed,
            sound_volume: 0.6,
        }
    }
}

/// Used to create initial global config
/// and then changes state to `GlobalState::MainMenu`
fn setup(
    mut commands: Commands,
    mut physics: ResMut<RapierConfiguration>,
    mut windows: Query<&mut Window>,
    mut global_state: ResMut<NextState<GlobalState>>,
) {
    // disable gravity because game is 2d top down
    physics.gravity = Vec2::ZERO;

    let mut camera_bundle = Camera2dBundle::default();
    // make everything smaller
    camera_bundle.projection.scale = 1.8;
    commands.spawn(camera_bundle);

    let game_settings = GameSettings::default();

    for mut window in windows.iter_mut() {
        window.present_mode = PresentMode::AutoVsync;
        window.mode = game_settings.window_mode;
    }

    commands.insert_resource(game_settings);

    global_state.set(GlobalState::MainMenu);
}
