use super::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use crate::CursorPosition;

fn map_click_to_gridcell(
    pos: Res<CursorPosition>,
    // pos: In<Option<Vec2>>,
    mut ev: EventReader<MouseButtonInput>,
    cell: Query<(&Pos, &Children), With<GridCell>>,
    mut textures: Query<&mut Visible>,
) {
    ev.iter()
        .filter(|ev| ev.state == ElementState::Pressed)
        .for_each(|_| {
            if let Some(coord) = **pos {
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
        // app.add_system(mouse.system().chain(map_click_to_gridcell.system()));
        app.add_system(map_click_to_gridcell.system());
    }
}
