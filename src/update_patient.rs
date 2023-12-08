use bevy::prelude::*;
use rand::Rng;

use crate::{patients::PatientRes, GameState, SCREEN_SIZE, gameplay::BacteriaComponent, SCREEN_CENTRE};

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_stopwatch, update_bacteria_velocities).run_if(in_state(GameState::Gameplay)));
    }
}

fn update_stopwatch (
    mut patients: ResMut<PatientRes>,
    time: Res<Time>,
) {
    let dt = time.delta();
    for patient in &mut patients.patients {
        patient.time_since_admission.tick(dt);
    }
}

fn update_bacteria_velocities (
    mut bact: Query<(&mut BacteriaComponent, &Transform)>,
) {

    let mut rng = rand::thread_rng();
    
    for (mut b, t) in bact.iter_mut() {

        if t.translation.x > (SCREEN_SIZE.x / 2.0) + SCREEN_CENTRE.x {

            b.velocity.x = -(b.velocity.x.abs() + 10.0)

        } else if -t.translation.x > (SCREEN_SIZE.x / 2.0) - SCREEN_CENTRE.x{

            b.velocity.x = b.velocity.x.abs() + 10.0

        } else {

            let x = rng.gen_range(-60.0..60.0);
            b.velocity.x += x;

        }

        if t.translation.y > (SCREEN_SIZE.y / 2.0) + SCREEN_CENTRE.y {

            b.velocity.y = -(b.velocity.y.abs() + 10.0)
            // move back to centre

        } else if -t.translation.y > (SCREEN_SIZE.y / 2.0 ) - SCREEN_CENTRE.y{

            // move back to centre
            b.velocity.y = b.velocity.y.abs() + 10.0

        } else {
            let y = rng.gen_range(-60.0..60.0);
            b.velocity.y += y;
        }

        if b.velocity.x.abs() >= 1000.0 {
            b.velocity.x *= 0.9;
        }

        if b.velocity.y.abs() >= 1000.0 {
            b.velocity.y *= 0.9;
        }

        if b.velocity.x.abs() <= 300.0 {
            b.velocity.x *= 1.1;
        }

        if b.velocity.y.abs() <= 300.0 {
            b.velocity.y *= 1.1;
        }

    }

}