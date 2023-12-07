use bevy::prelude::*;

use crate::{GameState, BACKGROUND_COLOUR, startup::GameTextures, update_patient::UpdatePlugin, patients::PatientRes};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Gameplay), (setup_camera, setup_sprites, add_patient))
            .add_plugins(UpdatePlugin)
            .insert_resource(PatientRes::new())
            ;
    }
}

fn display_bacteria (
    pts: Res<PatientRes>,
    bct: Query<>,
    mut cmd: Commands,
) {
    cmd.spawn((
        SpriteSheetBundle {
            texture_atlas: gt.bacteria.clone(),
            sprite: TextureAtlasSprite::new(1),
            transform: Transform {
                translation: Vec3::new(0.0, 80.0, 5.0),
                ..Default::default()
            },
            ..default()
        },
    ));
}

fn add_patient (
    mut pts: ResMut<PatientRes>,
) {
    pts.add_patient();
}

fn setup_camera (
    mut cmd: Commands,
) {
    cmd.spawn(Camera2dBundle::default());
    cmd.insert_resource(ClearColor(BACKGROUND_COLOUR));

}

fn setup_sprites (
    mut cmd: Commands,
    gt: Res<GameTextures>,
) {
    cmd.spawn((
        SpriteBundle {
            texture: gt.monitor.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..Default::default()
            },
            ..default()
        },
    ));
    cmd.spawn((
        SpriteBundle {
            texture: gt.monitor_bg.clone(),
            transform: Transform {
                    translation: Vec3::new(0.0, 120.0, 1.0),
                    ..Default::default()
                },
            ..default()
        },
    ));
}