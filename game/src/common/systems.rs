use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    // Add entities to the world
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            scale: Vec2::new(120.0, 5.0).extend(1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::Rgba {
                red: 0.8,
                green: 0.2,
                blue: 0.1,
                alpha: 0.5,
            },
            ..default()
        },
        ..default()
    });
}
