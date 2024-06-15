use bevy::prelude::*;

use super::bundles::EnemyBundle;

pub fn setup_enemies(mut commands: Commands) {
    // Add entities to the world
    commands.spawn(EnemyBundle::new(10.0, 10.0, "Enemy1", 20));
}

pub fn move_enemies(time: Res<Time>, mut query: Query<&mut Transform, With<EnemyBundle>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += time.delta_seconds();
    }
}
