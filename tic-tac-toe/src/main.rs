use bevy::prelude::*;

mod gridcell;

fn make_scene(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::build()
        .add_startup_system(make_scene.system())
        .add_plugins(DefaultPlugins)
        .run()
}
