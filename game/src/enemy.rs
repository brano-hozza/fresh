use bevy::prelude::*;

pub mod bundles;
pub mod components;
pub mod systems;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_enemies)
            .add_systems(Update, systems::move_enemies);
    }
}
