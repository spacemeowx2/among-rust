use bevy::prelude::*;

pub struct Player;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Position(pub Vec2);

pub struct Moving(pub bool);
