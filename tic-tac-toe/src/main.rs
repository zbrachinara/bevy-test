use bevy::prelude::*;

use bevy_svg::prelude::*;

mod gridcell;

fn make_scene(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let blue = SvgBuilder::from_file("tic-tac-toe/assets/blue_mark.svg")
        .position(Default::default())
        .build()
        .unwrap();
    commands.spawn_bundle(blue);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SvgPlugin)
        .add_startup_system(make_scene.system())
        .run()
}
