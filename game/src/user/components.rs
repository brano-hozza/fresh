use bevy::prelude::*;

#[derive(Component)]
pub struct Player(pub String);

#[derive(Component, Debug, PartialEq)]
pub enum PlayerState {
    Idle,
    Walking,
    Attacking,
    Sprinting,
    Interacting,
}
