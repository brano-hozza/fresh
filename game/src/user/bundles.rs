use bevy::prelude::*;

use crate::common::components::{
    AnimationIndices, AnimationTimer, Damage, EntityDirection, Health, Position,
};

use super::components::{Player, PlayerState};

#[derive(Bundle)]
pub struct UserBundle {
    pub position: Position,
    pub user: Player,
    pub health: Health,
    pub damage: Damage,
    pub sprite_bundle: SpriteSheetBundle,
    pub animation_timer: AnimationTimer,
    pub animation_indices: AnimationIndices,
    pub state: PlayerState,
    pub direction: EntityDirection,
}

impl UserBundle {
    pub fn new(
        x: f32,
        y: f32,
        name: &str,
        damage: i32,
        texture: Handle<Image>,
        atlas_layout: Handle<TextureAtlasLayout>,
    ) -> Self {
        /*
         * 0 - 3 = Down
         * 4 - 7 = Up
         * 8 - 11 = Left
         * 12 - 15 = Right
         */
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        UserBundle {
            position: Position { x, y },
            user: Player(String::from(name)),
            health: Health(100),
            damage: Damage(damage),
            sprite_bundle: SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_scale(Vec3::splat(6.0)),
                ..default()
            },
            animation_indices,
            state: PlayerState::Idle,
            direction: EntityDirection::Down,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        }
    }
}
