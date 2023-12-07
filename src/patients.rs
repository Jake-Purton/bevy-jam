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
        self.ph_mut = Mutation::random();
        self.o2_mut = Mutation::random();
    }
    pub fn new_random() -> Self {
        Self { temp_mut: Mutation::random(), ph_mut: Mutation::random(), o2_mut: Mutation::random() }
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
    pub fn new_random() -> Self {
        Self { bacteria: Bacteria::new_random(), temp: 0.0, ph: 0.0, o2: 0.0, time_since_admission: Stopwatch::new() }
    }
}

#[derive(Resource)]
pub struct PatientRes {
    pub patients: Vec<Patient>,
    patient_num: Option<usize>,
}

impl PatientRes {
    pub fn new() -> Self {
        PatientRes { patients: Vec::new(), patient_num: None }
    }

    pub fn next_patient(&mut self) {
        if self.patients.len() >= 1 {
            
            self.patient_num = Some((self.patient_num.unwrap() + 1) % self.patients.len());
        }
    }

    pub fn get_patient (&mut self) -> Option<&mut Patient> {
        if self.patients.len() >= 1 {
            Some(&mut self.patients[self.patient_num.unwrap()])
        } else {
            None
        }
    }

    pub fn add_patient (&mut self) {
        if let Some(a) = &mut self.patient_num {
            *a += 1;
        } else {
            self.patient_num = Some(0);
        }
        self.patients.push(Patient::new_random()) 

    }
}