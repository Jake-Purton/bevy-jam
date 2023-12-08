use bevy::prelude::*;

use crate::{GameState, BACKGROUND_COLOUR, SCREEN_CENTRE, startup::GameTextures, update_patient::UpdatePlugin, patients::PatientRes};
use rand::Rng;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Gameplay), (setup_camera, setup_sprites, add_patient))
            .add_systems(Update, display_bacteria.run_if(in_state(GameState::Gameplay)))
            .add_plugins(UpdatePlugin)
            .insert_resource(PatientRes::new())
            ;
    }
}

#[derive(Component)]
pub struct BacteriaComponent {
    pub velocity: Vec2,
    current_atlas: usize,
    patient_parent: usize,
}

fn display_bacteria (
    mut pts: ResMut<PatientRes>,
    mut bct: Query<(Entity, &mut Transform, &mut BacteriaComponent, &mut TextureAtlasSprite)>,
    mut cmd: Commands,
    gt: Res<GameTextures>,
    time: Res<Time>
) {

    let mut rng = rand::thread_rng();
    let mut count = 0;

    if let Some(c) = pts.get_patient_num() {
        for (entity, mut t, bacteria, mut tas) in bct.iter_mut() {

            if bacteria.patient_parent != c {
                cmd.entity(entity).despawn();
            } else {
                count += 1;
                t.translation += (bacteria.velocity * time.delta_seconds()).extend(0.0);

                let code = pts.get_patient().unwrap().get_bact_code();

                if bacteria.current_atlas != code {

                    tas.index = code

                }

            }
        }

        if let Some(pt) = pts.get_patient() {

            let diff = (pt.get_bact_num().floor() as u32) -(count as u32);

            if diff > 0 {
                for _ in 0..diff {

                    let x: f32 = rng.gen_range(-700.0..=700.0);
                    let y: f32 = rng.gen_range(SCREEN_CENTRE.y - 400.0..=SCREEN_CENTRE.y + 400.0);

                    println!("spawn bacterium");
                    cmd.spawn((
                        SpriteSheetBundle {
                            texture_atlas: gt.bacteria.clone(),
                            sprite: TextureAtlasSprite::new(0),
                            transform: Transform::from_translation(Vec3::new(x, y, 5.0)),
                            ..default()
                        },
                        BacteriaComponent {
                            velocity: Vec2::splat(0.0),
                            current_atlas: 0,
                            patient_parent: c,
                        }
                    ));
                }
            }

        }
        
    } else {
        for (entity, _, _, _) in bct.iter() {
            cmd.entity(entity).despawn();
        }
    }
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