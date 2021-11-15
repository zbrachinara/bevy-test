use bevy::prelude::*;

use bevy_svg::prelude::*;
use crate::gridcell::Pos;

mod gridcell;

fn make_scene(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.add(gridcell::AddGridCell{
        pos: Pos(0, 0),
        red: "tic-tac-toe/assets/red_mark.svg",
        blue: "tic-tac-toe/assets/blue_mark.svg",
        bbox: "tic-tac-toe/assets/empty_cell.svg",
    })

    // commands.spawn_bundle(blue);

}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SvgPlugin)
        .add_startup_system(make_scene.system())
        .run()
}
