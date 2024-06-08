use bevy::prelude::*;

use super::components::{Position, User};

#[derive(Bundle)]
pub struct UserBundle {
    pub position: Position,
    pub user: User,
}

impl Default for UserBundle {
    fn default() -> Self {
        UserBundle {
            position: Position { x: 0.0, y: 0.0 },
            user: User(String::from("default")),
        }
    }
}

impl UserBundle {
    pub fn new(x: f32, y: f32, name: &str) -> Self {
        UserBundle {
            position: Position { x, y },
            user: User(String::from(name)),
        }
    }
}
