use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{patients::PatientRes, GameState, SCREEN_SIZE, gameplay::BacteriaComponent, SCREEN_CENTRE, GROWTH_RATE, DIMINISH_RATE};

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_stopwatch, update_bacteria_velocities, update_bct_num).run_if(in_state(GameState::Gameplay)));
    }
}

fn update_stopwatch (
    mut patients: ResMut<PatientRes>,
    time: Res<Time>,
) {

    let mut rng = rand::thread_rng();

    let dt = time.delta();
    for patient in &mut patients.patients {
        if let Some(patient) = patient {

            patient.time_since_admission.tick(dt);
            patient.mutation_timer.tick(dt);
    
            if patient.mutation_timer.just_finished() {
                patient.bacteria.mutate();
    
                let duration = Duration::from_millis(rng.gen_range(10000..20000));
    
                patient.mutation_timer = Timer::new(duration, TimerMode::Once)
            }
        }

    }
}

fn update_bct_num (
    mut patients: ResMut<PatientRes>,
    time: Res<Time>,
    mut gs: ResMut<NextState<GameState>>,
) {

    let mut vec = Vec::new();
    let mut temp: [f32; 3] = [100.0, 100.0, 100.0];

    for (i, patient) in patients.patients.iter_mut().enumerate() {
        if let Some(patient) = patient {

            let a = patient.get_sliders();
    
            let mut delta_b = 0.0;
    
            if a.temp_mut == patient.bacteria.temp_mut {
                delta_b -= DIMINISH_RATE;
    
            } else {
                delta_b += GROWTH_RATE;
            }
    
            if a.hm_mut == patient.bacteria.hm_mut {
    
                delta_b -= DIMINISH_RATE;
            } else {
                delta_b += GROWTH_RATE;
            }
    
            if a.ph_mut == patient.bacteria.ph_mut {
    
                delta_b -= DIMINISH_RATE;
    
            } else {
                delta_b += GROWTH_RATE;
            }
    
            if a.o2_mut == patient.bacteria.o2_mut {
    
                delta_b -= DIMINISH_RATE;
    
            } else {
                delta_b += GROWTH_RATE;
            }
    
            let dt = time.delta_seconds();
    
            temp[i] = delta_b;
    
            patient.bacteria_num += delta_b * dt;
    
            if patient.bacteria_num <= 0.0 {
                vec.push(i);
            } else if patient.bacteria_num >= 120.0 {
                println!("loser");
                gs.set(GameState::Lose);
            }
        }
    }

    for i in vec {
        patients.remove_patient(i);
    }

    if let Some(a) = patients.get_patient_num() {

        println!("{}, {}", temp[a], patients.get_patient_num().unwrap());
    } else {
        println!("winner");
        gs.set(GameState::Win);
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