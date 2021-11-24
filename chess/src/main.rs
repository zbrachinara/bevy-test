
use bevy::prelude::*;

fn hello_world(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    println!("Hello world!");
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world.system())
        .run();
}
