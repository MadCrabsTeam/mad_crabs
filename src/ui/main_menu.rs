use bevy::{app::AppExit, prelude::*};

use crate::{utils::remove_all_with, GlobalState};

use super::{spawn_button, UiConfig, UiState};

pub struct UiMainMenuPlugin;

impl Plugin for UiMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiState::MainMenu)))
            .add_system(button_system.in_set(OnUpdate(UiState::MainMenu)))
            .add_system(remove_all_with::<UiMainMenuMarker>.in_schedule(OnExit(UiState::MainMenu)));
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct UiMainMenuMarker;

#[derive(Debug, Clone, Copy, Component)]
enum UiMainMenuButton {
    Start,
    Exit,
}

fn setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        .spawn(NodeBundle {
            style: config.menu_style.clone(),
            background_color: config.menu_color.into(),
            ..default()
        })
        .insert(UiMainMenuMarker)
        .with_children(|builder| {
            spawn_button(builder, &config, UiMainMenuButton::Start, UiMainMenuMarker);
            spawn_button(builder, &config, UiMainMenuButton::Exit, UiMainMenuMarker);
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut global_state: ResMut<NextState<GlobalState>>,
    mut interaction_query: Query<
        (&UiMainMenuButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.button_color_pressed.into();
                match button {
                    UiMainMenuButton::Start => {
                        global_state.set(GlobalState::InGame);
                    }
                    UiMainMenuButton::Exit => exit.send(AppExit),
                }
            }
            Interaction::Hovered => {
                *color = style.button_color_hover.into();
            }
            Interaction::None => {
                *color = style.button_color_normal.into();
            }
        }
    }
}
