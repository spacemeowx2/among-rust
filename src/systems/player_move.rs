use crate::{
    components::{Player, Position},
    plugins::input::UserInput,
};
use bevy::{math::vec3, prelude::*};

pub fn player_move(
    mut player: Query<(&Player, &mut Position, &mut Transform)>,
    input: Res<UserInput>,
) {
    for (_, mut position, mut transform) in player.iter_mut() {
        position.0 = position.0 + input.direction;
        transform.translation = vec3(position.0.x, position.0.y, 0.0)
    }
}
