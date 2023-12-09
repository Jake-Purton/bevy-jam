use bevy::{prelude::*, window::PrimaryWindow, sprite::collide_aabb::collide};

use crate::{GameState, WINDOW_SIZE, patients::PatientRes};

pub enum ButtonAction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct MyButton {
    pub size: Vec2,
    pub action: ButtonAction,
}

#[derive(Component)]
pub struct Slider {
    pub size: Vec2,
}

pub struct MyUIPlugin;

impl Plugin for MyUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (button_click_system).run_if(in_state(GameState::Gameplay)));
    }
}

fn button_click_system (
    mut buttons: Query<(&MyButton, &Transform, &mut TextureAtlasSprite)>,
    mouse: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
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
                        ButtonAction::Up => (),
                        ButtonAction::Down => (),
                    }


                }
            }
        }
    }

}