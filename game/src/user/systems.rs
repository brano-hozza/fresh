use crate::common::resources::GameTimer;
use bevy::prelude::*;

use super::{
    bundles::UserBundle,
    components::{Position, User},
    resources::MoveTimer,
};

pub fn print_position_system(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    query: Query<(&Position, &User)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (position, user) in &query {
            println!("{}: {} {}", user.0, position.x, position.y);
        }
    }
}

pub fn add_user_to_world(mut commands: Commands) {
    commands.spawn(UserBundle::new(20.0, -10.0, "user1"));
    commands.spawn(UserBundle::default());
}

pub fn update_positions(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut query: Query<&mut Position, With<User>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut position in &mut query {
            position.x += 1.0;
        }
    }
}
