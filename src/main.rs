#![allow(clippy::type_complexity)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_menu;
mod startup;
mod gameplay;
mod patients;
mod update_patient;

// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use gameplay::GameplayPlugin;
use main_menu::MenuPlugin;
use startup::StartupPlugin;

const BACKGROUND_COLOUR: Color = Color::rgb(0.05, 0.0, 0.1);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Menu,
    #[default]
    Gameplay,
}

fn main() {

    App::new()
        .add_state::<GameState>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(DefaultPlugins)
        .add_plugins(MenuPlugin)
        .add_plugins((StartupPlugin, GameplayPlugin))
        // .add_plugins(WorldInspectorPlugin::new())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}