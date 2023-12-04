use bevy::prelude::*;

use crate::{GameState, BACKGROUND_COLOUR};

const PLAY: &str = "Singleplayer";
const HOST: &str = "Host";
const JOIN: &str = "Join";
const EXIT: &str = "Exit";

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {

        app.add_systems(OnEnter(GameState::Menu), setup_menu);
    }
}

#[derive(Component)]
pub struct Menu;

fn setup_menu(mut commands: Commands) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOUR));
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                format!("{}\n", PLAY),
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::new(
                format!("{}\n", HOST),
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::new(
                format!("{}\n", JOIN),
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::new(
                format!("{}\n", EXIT),
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
        ]),
        Menu,
    ));
}