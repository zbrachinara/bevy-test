use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use std::path::{Path, PathBuf};

use crate::gridcell::Pos;
use bevy_svg::prelude::*;

mod gridcell;

struct MainCamera;

fn make_scene(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    let root = PathBuf::from("tic-tac-toe/assets");
    let (red, blue, bbox) = {
        let (mut red, mut blue, mut bbox) = (root.clone(), root.clone(), root.clone());

        red.push("red_mark.svg");
        blue.push("blue_mark.svg");
        bbox.push("empty_cell.svg");

        (red, blue, bbox)
    };

    (-1..=1).for_each(|x| {
        (-1..=1).for_each(|y| {
            commands.add(gridcell::AddGridCell {
                pos: Pos(x, y),
                red: red.clone(),
                blue: blue.clone(),
                bbox: bbox.clone(),
            })
        });
    });
}

fn mouse(windows: Res<Windows>, camera: Query<&Transform, With<MainCamera>>) -> Option<Vec2> {
    let window = windows.get_primary().unwrap();
    if let Some((pos, camera_transform)) = window.cursor_position().zip(camera.single().ok()) {
        let size = Vec2::new(window.width(), window.height());
        let pos = (pos - size / 2.0)
            // extend to vec4
            .extend(0.0)
            .extend(1.0);
        let world_pos = camera_transform.compute_matrix() * pos;

        Some(Vec2::new(world_pos.x, world_pos.y))
    } else {
        None
    }
}

fn map_click_to_gridcell(pos: In<Option<Vec2>>, mut ev: EventReader<MouseButtonInput>) {
    ev.iter().for_each(|ev| {
        if let Some(pos) = pos.0 {
            println!("Cursor located in-world at {}, {}", pos.x, pos.y);
        }
    })
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SvgPlugin)
        .add_startup_system(make_scene.system())
        .add_system(mouse.system().chain(map_click_to_gridcell.system()))
        .run()
}
