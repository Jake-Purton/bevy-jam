use bevy::prelude::*;

use crate::{GameState, BACKGROUND_COLOUR};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), setup_camera);
    }
}

fn setup_camera (
    mut cmd: Commands,
) {
    cmd.spawn(Camera2dBundle::default());
    cmd.insert_resource(ClearColor(BACKGROUND_COLOUR));

}
