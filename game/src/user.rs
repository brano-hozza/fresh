use bevy::prelude::*;
use resources::MoveTimer;

pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;

pub struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.insert_resource(MoveTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Startup, systems::add_user_to_world)
            .add_systems(
                Update,
                (systems::print_position_system, systems::update_positions),
            );
    }
}
