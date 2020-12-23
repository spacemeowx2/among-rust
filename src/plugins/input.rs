use bevy::{math::vec2, prelude::*};

#[derive(Default, Debug)]
pub struct UserInput {
    pub direction: Vec2,
}

#[derive(Debug, Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(InputPlugin)
            .add_resource(UserInput::default())
            .add_system(keyboard_system.system());
    }
}

pub fn keyboard_system(keyboard_input: Res<Input<KeyCode>>, mut input: ResMut<UserInput>) {
    let jp_right = (keyboard_input.just_pressed(KeyCode::Right)
        || keyboard_input.just_pressed(KeyCode::D)) as i32;
    let jp_left = (keyboard_input.just_pressed(KeyCode::Left)
        || keyboard_input.just_pressed(KeyCode::A)) as i32;
    let jp_up = (keyboard_input.just_pressed(KeyCode::Up)
        || keyboard_input.just_pressed(KeyCode::W)) as i32;
    let jp_down = (keyboard_input.just_pressed(KeyCode::Down)
        || keyboard_input.just_pressed(KeyCode::S)) as i32;

    let right =
        (keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D)) as i32;
    let left = (keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A)) as i32;
    let up = (keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W)) as i32;
    let down = (keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S)) as i32;

    let kx = (right | jp_right) - (left | jp_left);
    let ky = (up | jp_up) - (down | jp_down);

    input.direction = vec2(kx as f32, ky as f32);
}
