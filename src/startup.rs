use bevy::prelude::*;

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

    window.resolution.set(1600.0, 1080.0);
    window.resizable = true;

}

#[derive(Resource)]
pub struct GameTextures {
    pub bacteria: Handle<TextureAtlas>,
    pub monitor: Handle<Image>,
    pub monitor_bg: Handle<Image>,
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


    let gt = GameTextures {
        bacteria: bt_atlas_handle,
        monitor: mon_handle,
        monitor_bg: mon_bg,
    };

    cmd.insert_resource(gt);

}