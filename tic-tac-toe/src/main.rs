use bevy::prelude::*;
use std::path::{Path, PathBuf};
use bevy::input::mouse::MouseButtonInput;
use bevy::render::camera::Camera;

use crate::gridcell::Pos;
use bevy_svg::prelude::*;

mod gridcell;

fn make_scene(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let root = PathBuf::from("tic-tac-toe/assets");
    let (red, blue, bbox) = {
        let (mut red, mut blue, mut bbox) = (root.clone(), root.clone(), root.clone());

        red.push("red_mark.svg");
        blue.push("blue_mark.svg");
        bbox.push("empty_cell.svg");

        (red, blue, bbox)
    };

    (0..3).for_each(|x| {
        (0..3).for_each(|y| {
            commands.add(gridcell::AddGridCell {
                pos: Pos(x, y),
                red: red.clone(),
                blue: blue.clone(),
                bbox: bbox.clone(),
            })
        });
    });

    // commands.spawn_bundle(blue);
}

fn mouse(windows: Res<Windows>, camera: Query<&Transform, With<Camera>>) -> Vec2 {
    Vec2::new(0.0, 0.0)
}

fn map_click_to_gridcell(pos: In<Vec2>, ev: EventReader<MouseButtonInput>) {
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SvgPlugin)
        .add_startup_system(make_scene.system())
        .add_system(mouse.system().chain(map_click_to_gridcell.system()))
        .run()
}
