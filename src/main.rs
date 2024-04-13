mod player;
mod enviroment;

use bevy::prelude::*;

const SPRITE_SIZE: f32 = 8.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
             .set(ImagePlugin::default_nearest())
        )
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enviroment::EnviromentPlugin)
        .add_systems(Startup, setup_world)
        .run();
}

fn setup_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
