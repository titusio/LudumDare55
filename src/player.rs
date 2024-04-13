use bevy::prelude::*;
use crate::SPRITE_SIZE;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
        ;
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("wizard.png"),
        transform: Transform::from_scale(Vec3::splat(SPRITE_SIZE)),
        ..Default::default()
        },
        Player
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>
) {
    let mut input = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        input.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    input = input.normalize_or_zero() * 500.0 * time.delta_seconds();

    for mut transform in query.iter_mut() {
        // moving the player
        transform.translation.x += input.x;
        transform.translation.y += input.y;

        // flipping the player sprite
        if input.x < 0.0 && transform.scale.x < 0.0 {
            transform.scale.x = transform.scale.x.abs();
        }
        else if input.x > 0.0 && transform.scale.x > 0.0 {
            transform.scale.x *= -1.0;
        } 
    }
}
