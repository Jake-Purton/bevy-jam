use bevy::{prelude::*, window::PrimaryWindow};

use crate::{GameState, WINDOW_SIZE, patients::PatientRes, SLIDER_POSITIONS, SLIDER_BOTTOM_Y, SLIDER_TOP_Y};

pub enum SliderType {
    Ph,
    O2,
    Temp,
    Humidity,
}

pub enum ButtonAction {
    Left,
    Right,
    // Up,
    // Down,
}

#[derive(Component)]
pub struct MyButton {
    pub size: Vec2,
    pub action: ButtonAction,
}

#[derive(Component)]
pub struct Slider {
    pub size: Vec2,
    pub stype: SliderType,
}

pub struct MyUIPlugin;

impl Plugin for MyUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (button_click_system, slider_system, move_slider).run_if(in_state(GameState::Gameplay)));
    }
}

fn button_click_system (
    mut buttons: Query<(&MyButton, &Transform, &mut TextureAtlasSprite)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>,
    mut patients: ResMut<PatientRes>,
) {


    if let Some(position) = q_windows.single().cursor_position() {

        let posx = position.x - 0.5 * WINDOW_SIZE.x;
        let posy = -(position.y - 0.5 * WINDOW_SIZE.y);

        let pos = Vec2::new(posx, posy - 20.0);


        for mut button in buttons.iter_mut() {

            let y = ( -0.5 *(button.0.size.y) + button.1.translation.y, 0.5 *(button.0.size.y) + button.1.translation.y);
            let x = ( -0.5 *(button.0.size.x) + button.1.translation.x, 0.5 *(button.0.size.x) + button.1.translation.x);

            button.2.index = 0;

            if pos.x > x.0 && pos.x < x.1 && pos.y > y.0 && pos.y < y.1 + 10.0 {
                if mouse.pressed(MouseButton::Left) {
                    button.2.index = 1;
                }
                if mouse.just_pressed(MouseButton::Left) {

                    match button.0.action {
                        ButtonAction::Left => {
                            patients.previous_patient();
                            println!("next patient");
                        },
                        ButtonAction::Right => {
                            patients.next_patient();
                            println!("next patient");
                        },
                        // ButtonAction::Up => (),
                        // ButtonAction::Down => (),
                    }


                }
            }
        }
    }

}

fn move_slider (
    mut patients: ResMut<PatientRes>,
    mut sliders: Query<(&mut Transform, &Slider)>
) {

    let p = patients.get_patient().unwrap();

    for mut slider in sliders.iter_mut() {
        match slider.1.stype {
            SliderType::Ph => {
                slider.0.translation.y = p.ph;
            },
            SliderType::O2 => {
                slider.0.translation.y = p.o2;
                
            },
            SliderType::Temp => {
                slider.0.translation.y = p.temp;
                
            },
            SliderType::Humidity => {
                slider.0.translation.y = p.humidity;
            
            },
        }
    }

}

fn slider_system (

    q_windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>,
    mut patients: ResMut<PatientRes>,

) {

    if mouse.pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let posx = position.x - 0.5 * WINDOW_SIZE.x;
            let posy = -(position.y - 0.5 * WINDOW_SIZE.y);
    
            let mut pos = Vec2::new(posx, posy - 30.);

            for (min, max, stype) in SLIDER_POSITIONS {

                if pos.x >= min && pos.x <= max && pos.y >= SLIDER_BOTTOM_Y - 50.0 && pos.y <= SLIDER_TOP_Y + 50.0 {

                    pos.y = SLIDER_BOTTOM_Y.max(pos.y);
                    pos.y = SLIDER_TOP_Y.min(pos.y);

                    match stype {
                        SliderType::Ph => {
                            patients.get_patient().unwrap().ph = pos.y;
                        },
                        SliderType::O2 => {
                            patients.get_patient().unwrap().o2 = pos.y;
                        },
                        SliderType::Temp => {
                            patients.get_patient().unwrap().temp = pos.y;
                        },
                        SliderType::Humidity => {
                            patients.get_patient().unwrap().humidity = pos.y;
                        },
                    }

                }

            }

        }
    }
    
}