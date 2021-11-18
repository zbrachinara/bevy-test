use super::prelude::*;
use std::path::PathBuf;

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

fn hide_markers(mut q: Query<&mut Visible, Added<crate::gridcell::Marker>>) {
    q.iter_mut().for_each(|mut visible| {
        visible.is_visible = false;
    })
}

pub struct GameInit;
impl Plugin for GameInit {
    fn build(&self, app: &mut AppBuilder) {
        static POST_SVG: &str = "POST_SVG";
        app.add_startup_system(make_scene.system())
            .add_stage(POST_SVG, SystemStage::single_threaded())
            .add_system_to_stage(POST_SVG, hide_markers.system());
    }
}