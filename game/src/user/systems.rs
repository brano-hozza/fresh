use crate::common::components::{AnimationIndices, AnimationTimer, EntityDirection, Position};
use bevy::prelude::*;

use super::{
    bundles::UserBundle,
    components::{Player, PlayerState},
    resources::MoveTimer,
};

pub fn add_user_to_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprite_sheets/character_basic.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(16.0, 16.0),
        4,
        4,
        Some(Vec2::new(32.0, 32.0)),
        Some(Vec2::new(16.0, 16.0)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(UserBundle::new(
        20.0,
        10.0,
        "user1",
        10,
        texture,
        texture_atlas_layout,
    ));
}

pub fn update_positions(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut query: Query<&mut Position, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut position in &mut query {
            position.x += 1.0;
        }
    }
}

pub fn animate_player(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlas,
            &EntityDirection,
            &PlayerState,
        ),
        With<Player>,
    >,
) {
    for (indices, mut timer, mut atlas, direction, state) in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            if *state == PlayerState::Idle {
                atlas.index = indices.first;
                continue;
            }
            if *state == PlayerState::Attacking {
                todo!("Implement attack animation");
            }
            if *state == PlayerState::Walking {
                atlas.index = if atlas.index >= indices.last || atlas.index < indices.first {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

// Change direction property and indices based on keyboard input
pub fn change_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut EntityDirection, &mut AnimationIndices), With<Player>>,
) {
    for (mut direction, mut indicies) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::ArrowUp) && *direction != EntityDirection::Up {
            *direction = EntityDirection::Up;
            indicies.first = 4;
            indicies.last = 7;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) && *direction != EntityDirection::Down {
            *direction = EntityDirection::Down;
            indicies.first = 0;
            indicies.last = 3;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) && *direction != EntityDirection::Left {
            *direction = EntityDirection::Left;
            indicies.first = 8;
            indicies.last = 11;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) && *direction != EntityDirection::Right
        {
            *direction = EntityDirection::Right;
            indicies.first = 12;
            indicies.last = 15;
        }
    }
}

pub fn change_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerState, With<Player>>,
) {
    for mut state in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            *state = PlayerState::Attacking;
        }
        if keyboard_input.any_pressed(vec![
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
        ]) {
            *state = PlayerState::Walking;
        } else {
            *state = PlayerState::Idle;
        }
    }
}
