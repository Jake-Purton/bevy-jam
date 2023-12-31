#![allow(clippy::type_complexity)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_menu;
mod startup;
mod gameplay;
mod patients;
mod update_patient;
mod ui;
mod systems;

// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::{prelude::*, asset::AssetMetaCheck};
use gameplay::GameplayPlugin;
use main_menu::MenuPlugin;
use startup::StartupPlugin;
use ui::{MyUIPlugin, SliderType};

const BACKGROUND_COLOUR: Color = Color::rgb(0.05, 1.0, 0.1);
const SCREEN_CENTRE: Vec2 = Vec2::new(0.0, 120.0);
const SCREEN_SIZE: Vec2 = Vec2::new(944.0, 600.0);
const WINDOW_SIZE: Vec2 = Vec2::new(1600.0, 1080.0);
const SLIDER_TOP_Y: f32 = -361.0;
const SLIDER_BOTTOM_Y: f32 = -482.0;
const SLIDER_MIDDLE: f32 = (SLIDER_TOP_Y + SLIDER_BOTTOM_Y) / 2.0;
const GROWTH_RATE: f32 = 1.0;
const DIMINISH_RATE: f32 = 0.5;
// (min_x, max_x, type)
const SLIDER_POSITIONS: [(f32, f32, SliderType); 4] = [
    (-510.0-46.0, 46.0-510.0, SliderType::Temp), 
    (-382.0-46.0, 46.0-382.0, SliderType::O2), 
    (-254.0-46.0, 46.0-254.0, SliderType::Ph), 
    (-126.0-46.0, 46.0-126.0, SliderType::Humidity)
    ];
// const SCREEN_SIZE: Vec2 = Vec2::new(20.0, 20.0);

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    Menu,
    #[default]
    Gameplay,
    Win,
    Lose
}

fn main() {

    App::new()
        .add_state::<GameState>()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins)
        .add_plugins(MenuPlugin)
        .add_plugins((StartupPlugin, GameplayPlugin, MyUIPlugin))
        // .add_plugins(WorldInspectorPlugin::new())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}