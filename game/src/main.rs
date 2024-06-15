use bevy::prelude::*;

mod common;
mod enemy;
mod user;

use common::resources::GameTimer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Basic Example - Press Space to change Texture and H to show/hide tilemap.",
                ),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_plugins((enemy::EnemyPlugin, user::UserPlugin))
        .add_systems(Startup, common::systems::setup)
        .run();
}
