use super::prelude::*;
use bevy::input::ElementState;
use bevy::input::mouse::MouseButtonInput;

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

fn map_click_to_gridcell(
    pos: In<Option<Vec2>>,
    mut ev: EventReader<MouseButtonInput>,
    cell: Query<(&Pos, &Children), With<gridcell::GridCell>>,
    mut textures: Query<&mut Visible>,
) {
    ev.iter()
        .filter(|ev| ev.state == ElementState::Pressed)
        .for_each(|_| {
            if let Some(coord) = pos.0 {
                let child = cell
                    .iter()
                    .find(|(cell_pos, _)| cell_pos == &&coord_to_pos(coord))
                    .map(|(_, children)| children.iter().nth(0).unwrap())
                    .unwrap();
                textures.get_mut(*child).unwrap().is_visible = true;
            }
        })
}

pub struct GameLogic;
impl Plugin for GameLogic {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(mouse.system().chain(map_click_to_gridcell.system()));
    }
}