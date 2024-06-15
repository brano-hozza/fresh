use bevy::prelude::*;

use crate::common::components::{Damage, Health, Position};

use super::components::Enemy;

#[derive(Component)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub position: Position,
    pub health: Health,
    pub damage: Damage,
}

impl EnemyBundle {
    pub fn new(x: f32, y: f32, name: &str, damage: i32) -> Self {
        EnemyBundle {
            position: Position { x, y },
            enemy: Enemy(String::from(name)),
            health: Health(100),
            damage: Damage(damage),
        }
    }
}
