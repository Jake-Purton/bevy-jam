use bevy::prelude::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, set_res);
    }
}

fn set_res(
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    window.resolution.set(1600.0, 800.0);
    window.resizable = false;
}