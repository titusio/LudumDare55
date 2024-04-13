use bevy::prelude::*;
use crate::SPRITE_SIZE;

pub struct EnviromentPlugin;

impl Plugin for EnviromentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_ground);
    }
}

fn create_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ground.png"),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SIZE)).with_translation(Vec3::new(0.0, 0.0, -1.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(600.0, 90.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.0
        }
    ));
}
