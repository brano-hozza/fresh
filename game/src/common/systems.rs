use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    // Add entities to the world
    commands.spawn(Camera2dBundle::default());
}
