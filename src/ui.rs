use bevy::{prelude::*, window::PrimaryWindow, sprite::collide_aabb::collide};

use crate::{GameState, SCREEN_SIZE};

#[derive(Component)]
pub struct MyButton {
    pub size: Vec2,
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



) {

    if mouse.pressed(MouseButton::Left) {

        if let Some(position) = q_windows.single().cursor_position() {

            let posx = 

            for mut button in buttons.iter_mut() {

                if collide(position.extend(1.0), Vec2::splat(5.0), button.1.translation, button.0.size).is_some() {


                    println!("here1");

                    button.2.index = 1;

                }

            }
        } else {

            println!("you missed")

        }
    }

}