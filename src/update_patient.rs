use bevy::prelude::*;

use crate::{patients::PatientRes, GameState};

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_stopwatch.run_if(in_state(GameState::Gameplay)));
    }
}

fn update_stopwatch (
    mut patients: ResMut<PatientRes>,
    time: Res<Time>,
) {
    let dt = time.delta();
    for patient in &mut patients.patients {
        patient.tick(dt);
    }
}