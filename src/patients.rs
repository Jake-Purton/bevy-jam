use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use rand::prelude::*;

pub enum Mutation {
    High,
    None,
    Low,
}

impl Mutation {
    fn random() -> Mutation {
        let mut rng = rand::thread_rng();
        let a: u8 = rng.gen_range(0..=2);
        match a {
            0 => Mutation::Low,
            1 => Mutation::None,
            2 => Mutation::High,
            _ => panic!("rng generated a bad number"),
        }
    }
}

pub struct Bacteria {
    temp_mut: Mutation,
    ph_mut: Mutation,
    o2_mut: Mutation,
}

impl Bacteria {
    fn mutate (&mut self) {
        self.temp_mut = Mutation::random();
    }
}

pub struct Patient {
    bacteria: Bacteria,
    temp: f32,
    ph: f32,
    o2: f32,
    time_since_admission: Stopwatch,
}

impl Patient {
    pub fn tick(&mut self, delta: Duration) {
        self.time_since_admission.tick(delta);
    }
}

#[derive(Resource)]
pub struct PatientRes {
    pub patients: Vec<Patient>,
    patient_num: usize,
}

impl PatientRes {
    fn new() -> Self {
        PatientRes { patients: Vec::new(), patient_num: 0 }
    }

    fn next_patient(&mut self) {
        if self.patients.len() >= 1 {
            self.patient_num = (self.patient_num + 1) % self.patients.len();
        }
    }

    fn get_patient (&mut self) -> Option<&mut Patient> {
        if self.patients.len() >= 1 {
            Some(&mut self.patients[self.patient_num])
        } else {
            None
        }
    }
}