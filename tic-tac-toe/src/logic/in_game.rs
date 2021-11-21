use super::prelude::*;
use array2d::Array2D;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use std::iter::once;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Turn(Player);
impl Deref for Turn {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Turn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
enum Winner {
    Some(Player),
    None,
    Draw,
}

fn click_gridcell(
    pos: Res<CursorPosition>,
    mut ev: EventReader<MouseButtonInput>,
    mut cell: Query<(&Pos, &Children, &mut Option<Player>), With<GridCell>>,
    mut textures: Query<&mut Visible>,
    mut turn: ResMut<Turn>,
) -> bool {
    let mut updated = false;

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

                    match *owner {
                        None => {
                            textures.get_mut(*tex_entity).unwrap().is_visible = true;
                            *owner = Some(turn.0.clone());
                            turn.switch();
                            updated = true;
                        }
                        Some(_) => {}
                    }
                }
            }
        });

    updated
}

fn check_winner(updated: In<bool>, cells: Query<(&Pos, &Option<Player>), With<GridCell>>) {
    if updated.0 {
        let grid: Array2D<Option<Player>> = unsafe {
            let mut grid = Array2D::filled_with(None, 3, 3);
            cells.iter().for_each(|(Pos(x, y), player)| {
                grid.get_mut((x + 1) as usize, (y + 1) as usize)
                    .map(|p| *p = player.clone());
            });
            grid
        };

        println!("State of board: {:?}", grid);

        println!("{:?}", has_winner(grid));
    }
}

fn slice_all<T: Eq>(slice: &[T]) -> Option<&T> {
    if slice.windows(2).all(|t| t[0] == t[1]) {
        Some(&slice[0])
    } else {
        None
    }
}

fn has_winner(board: Array2D<Option<Player>>) -> Winner {
    let winning_sets = (board.rows_iter().map(|it| it.collect::<Vec<_>>()))
        .chain(board.columns_iter().map(|it| it.collect::<Vec<_>>()))
        .chain(once(
            (0..3).map(|i| board.get(i, i).unwrap()).collect::<Vec<_>>(),
        ))
        .chain(once(
            (0..3)
                .map(|i| board.get(i, 2 - i).unwrap())
                .collect::<Vec<_>>(),
        ));

    // println!(
    //     "Everything that wins: {:?}",
    //     winning_sets.collect::<Vec<_>>()
    // );

    let mut filled = true;
    for set in winning_sets {
        if let Some(Some(player)) = slice_all(set.as_slice()) {
            return Winner::Some(player.clone());
        }
        if set.contains(&&None) {
            filled = false;
        }
    }

    if filled {
        Winner::Draw
    } else {
        Winner::None
    }
}

pub struct GameLogic;
impl Plugin for GameLogic {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Turn(Player::Red))
            .add_system(click_gridcell.system().chain(check_winner.system()));
    }
}
