use super::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use crate::CursorPosition;
use crate::Player;

fn click_gridcell(
    pos: Res<CursorPosition>,
    mut ev: EventReader<MouseButtonInput>,
    mut cell: Query<(&Pos, &Children, &mut Option<Player>), With<GridCell>>,
    mut textures: Query<&mut Visible>
) {
    ev.iter()
        .filter(|ev| ev.state == ElementState::Pressed)
        .for_each(|_| {
            if let Some(coord) = **pos {

                let child = cell
                    .iter_mut()
                    .find(|(cell_pos, _, _)| cell_pos == &&coord_to_pos(coord))
                    .unwrap();
                let (tex_red, tex_blue) = {
                    let mut iter = child.1.iter();
                    (iter.next().unwrap(), iter.next().unwrap())
                };
                textures.get_mut(*tex_red).unwrap().is_visible = true;
            }
        })
}

pub struct GameLogic;
impl Plugin for GameLogic {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(click_gridcell.system());
    }
}
