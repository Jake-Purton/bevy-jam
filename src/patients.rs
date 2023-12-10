use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use rand::prelude::*;

use crate::{SLIDER_TOP_Y, SLIDER_BOTTOM_Y};

#[derive(Debug, PartialEq)]
pub enum Mutation {
    High,
    None,
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
    pub temp_mut: Mutation,

    // high = red
    // low = blue
    pub ph_mut: Mutation,

    // high = triangle
    // low = circle
    pub o2_mut: Mutation,

    // lines = humid = high
    // no lines = not humid = low
    pub hm_mut: Mutation,
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
    pub bacteria_num: f32,
    pub temp: f32,
    pub ph: f32,
    pub o2: f32,
    pub humidity: f32,
    pub time_since_admission: Stopwatch,
    pub mutation_timer: Timer,
}

impl Patient {
    pub fn new_random() -> Self {
        Self { 
            bacteria: Bacteria::new_random(), 
            bacteria_num: 10.0, 
            temp: SLIDER_TOP_Y, 
            ph: SLIDER_TOP_Y, 
            o2: SLIDER_TOP_Y, 
            humidity: SLIDER_TOP_Y, 
            time_since_admission: Stopwatch::new() ,
            mutation_timer: Timer::new(Duration::from_secs(10), TimerMode::Once),
        }
    }
    pub fn get_bact_num(&self) -> f32 {
        self.bacteria_num.clone()
    }

    pub fn turn_values_into_enum (val: f32) -> Mutation {

        let a =  (val - SLIDER_BOTTOM_Y) / (SLIDER_TOP_Y - SLIDER_BOTTOM_Y);

        if a < 0.33 {
            Mutation::Low
        } else if a < 0.66 {
            Mutation::None
        } else {
            Mutation::High
        }

    }

    pub fn get_sliders(&self) -> Bacteria {

        Bacteria {
            temp_mut: Self::turn_values_into_enum(self.temp),
            ph_mut: Self::turn_values_into_enum(self.ph),
            o2_mut: Self::turn_values_into_enum(self.o2),
            hm_mut: Self::turn_values_into_enum(self.humidity),
        }

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

    pub fn previous_patient(&mut self) {

        let a = self.patients.len();

        if a >= 1 {
            
            self.patient_num = Some((self.patient_num.unwrap() + (a-1)) % a);
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