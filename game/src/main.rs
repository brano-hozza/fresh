use bevy::prelude::*;

mod common;
mod user;

use common::resources::GameTimer;

fn main() {
    App::new()
        .insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_plugins((DefaultPlugins, user::UserPlugin))
        .add_systems(Startup, common::systems::setup)
        .run();
}
