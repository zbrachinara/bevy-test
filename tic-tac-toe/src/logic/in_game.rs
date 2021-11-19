use super::prelude::*;
use crate::CursorPosition;
use crate::Player;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;

#[derive(Debug)]
struct Turn(Player);

fn click_gridcell(
    pos: Res<CursorPosition>,
    mut ev: EventReader<MouseButtonInput>,
    mut cell: Query<(&Pos, &Children, &mut Option<Player>), With<GridCell>>,
    mut textures: Query<&mut Visible>,
    mut turn: ResMut<Turn>,
) {
    ev.iter()
        .filter(|ev| ev.state == ElementState::Pressed)
        .for_each(|_| {
            if let Some(coord) = **pos {
                if let Some((_, child, mut owner)) = cell
                    .iter_mut()
                    .find(|(cell_pos, _, _)| **cell_pos == coord_to_pos(coord))
                {
                    let tex_entity = {
                        let mut child = child.iter();
                        let textures = (child.next().unwrap(), child.next().unwrap());
                        match *turn {
                            Turn(Player::Red) => textures.0,
                            Turn(Player::Blue) => textures.1,
                        }
                    };

                    textures.get_mut(*tex_entity).unwrap().is_visible = true;
                }
            }
        })
}

pub struct GameLogic;
impl Plugin for GameLogic {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Turn(Player::Red))
            .add_system(click_gridcell.system());
    }
}
