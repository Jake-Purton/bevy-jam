use bevy::prelude::*;

use crate::WINDOW_SIZE;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (set_res, setup_textures));
    }
}

fn set_res(
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    window.resolution.set(WINDOW_SIZE.x, WINDOW_SIZE.y);
    window.resizable = true;

}

#[derive(Resource)]
pub struct GameTextures {
    pub bacteria: Handle<TextureAtlas>,
    pub monitor: Handle<Image>,
    pub red_light: Handle<Image>,
    pub green_light: Handle<Image>,
    pub monitor_bg: Handle<Image>,
    pub arrow_left: Handle<TextureAtlas>,
    pub arrow_right: Handle<TextureAtlas>,
    pub button_slider: Handle<TextureAtlas>,
}

fn setup_textures (
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let bt_handle = asset_server.load("bacteria-sheet.png");
    let bt_atlas =
        TextureAtlas::from_grid(bt_handle, Vec2::new(120.0, 120.0), 4, 4, None, None);
    let bt_atlas_handle = texture_atlases.add(bt_atlas);

    let mon_handle = asset_server.load("monitor.png");
    let mon_bg = asset_server.load("screen.png");

    let leftarr_handle = asset_server.load("arrow_left-sheet.png");
    let leftarr_atlas =
        TextureAtlas::from_grid(leftarr_handle, Vec2::new(40.0, 48.0), 2, 1, None, None);
    let leftarr_atlas_handle = texture_atlases.add(leftarr_atlas);

    let rightarr_handle = asset_server.load("arrow_right-sheet.png");
    let rightarr_atlas =
        TextureAtlas::from_grid(rightarr_handle, Vec2::new(40.0, 48.0), 2, 1, None, None);
    let rightarr_atlas_handle = texture_atlases.add(rightarr_atlas);

    let slider_handle = asset_server.load("button_slider-sheet.png");
    let slider_atlas =
        TextureAtlas::from_grid(slider_handle, Vec2::new(92.0, 36.0), 2, 1, None, None);
    let slider_atlas_handle = texture_atlases.add(slider_atlas);

    let rbt = asset_server.load("red_light.png");
    let gbt = asset_server.load("green_light.png");

    let gt = GameTextures {
        bacteria: bt_atlas_handle,
        monitor: mon_handle,
        monitor_bg: mon_bg,
        arrow_left: leftarr_atlas_handle,
        arrow_right: rightarr_atlas_handle,
        button_slider: slider_atlas_handle,
        red_light: rbt,
        green_light: gbt,
    };

    cmd.insert_resource(gt);

}