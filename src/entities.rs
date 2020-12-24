use crate::components::*;
use bevy::prelude::*;

pub fn create_player<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Player, Position::default(), Moving(false)))
}
