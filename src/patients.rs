use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use rand::prelude::*;

#[derive(Debug)]
pub enum Mutation {
    High,
    Low,
}

impl Mutation {
    fn random() -> Mutation {
        let mut rng = rand::thread_rng();
        let a: u8 = rng.gen_range(0..=1);
        match a {
            0 => Mutation::Low,
            1 => Mutation::High,
            _ => panic!("rng generated a bad number"),
        }
    }
}
#[derive(Debug)]
pub struct Bacteria {
    // high = hot
    // low = cold
    temp_mut: Mutation,

    // high = red
    // low = blue
    ph_mut: Mutation,

    // high = triangle
    // low = circle
    o2_mut: Mutation,
    
    // lines = humid = high
    // no lines = not humid = low
    hm_mut: Mutation,
}

impl Bacteria {
    pub fn mutate (&mut self) {
        self.temp_mut = Mutation::random();
        self.ph_mut = Mutation::random();
        self.o2_mut = Mutation::random();
    }
    pub fn new_random() -> Self {
        Self { temp_mut: Mutation::random(), ph_mut: Mutation::random(), o2_mut: Mutation::random(), hm_mut: Mutation::random() }
    }
    pub fn code(&self) -> usize {

        let mut code = 0;

        let t: usize = if matches!(self.temp_mut, Mutation::High) {
            //hot
            0
        } else {
            //cold
            1
        };

        let ph: usize = if matches!(self.ph_mut, Mutation::Low) {
            //blue
            1
        } else {
            //red
            0
        };

        let o2: usize = if matches!(self.o2_mut, Mutation::High) {
            // high oxygen / triangular
            1
        } else {
            // low oxygen / circular
            0
        };

        let hm: usize = if matches!(self.hm_mut, Mutation::High) {
            1
        } else {
            0
        };

        code += 4*t;
        code += 2*ph;
        code += 8*o2;
        code += hm;
        code

    }
}

pub struct Patient {
    pub bacteria: Bacteria,
    bacteria_num: f32,
    temp: f32,
    ph: f32,
    o2: f32,
    pub time_since_admission: Stopwatch,
}

impl Patient {
    pub fn tick(&mut self, delta: Duration) {
        self.time_since_admission.tick(delta);
    }
    pub fn new_random() -> Self {
        Self { bacteria: Bacteria::new_random(), bacteria_num: 2.0, temp: 0.0, ph: 0.0, o2: 0.0, time_since_admission: Stopwatch::new() }
    }
    pub fn get_bact_num(&self) -> f32 {
        self.bacteria_num.clone()
    }
    // beatles refrence for dan (get back)
    pub fn get_bact_code (&self) -> usize {
        self.bacteria.code()
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

    pub fn get_patient_num(&self) -> Option<usize> {
        self.patient_num
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