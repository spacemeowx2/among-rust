use crate::{components::*, plugins::input::UserInput};
use bevy::{math::vec3, prelude::*};
use std::f32::consts::PI;

pub fn player_move(
    mut player: Query<(&Player, &mut Position, &mut Transform, &mut Moving)>,
    input: Res<UserInput>,
) {
    for (_, mut position, mut transform, mut moving) in player.iter_mut() {
        moving.0 = input.direction != Vec2::zero();
        if moving.0 {
            position.0 = position.0 + input.direction * 5.0;
            transform.translation = vec3(position.0.x, position.0.y, 0.0);
            if input.direction.x > 0.0 {
                transform.rotation = Quat::from_rotation_y(0.0)
            } else if input.direction.x < 0.0 {
                transform.rotation = Quat::from_rotation_y(PI)
            }
        }
    }
}
