use bevy::{prelude::Resource, time::Timer};

#[derive(Resource)]
pub struct GameTimer(pub Timer);
