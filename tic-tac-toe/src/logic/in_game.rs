use super::prelude::*;
use crate::Orientation::Vertical;
use array2d::Array2D;
use bevy::app::Events;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use std::iter::once;
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

struct PostGameText;

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

fn check_winner(
    updated: In<bool>,
    cells: Query<(&Pos, &Option<Player>), With<GridCell>>,
    mut state: ResMut<State<GameState>>,
    commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    if updated.0 {
        let grid: Array2D<Option<Player>> = {
            let mut grid = Array2D::filled_with(None, 3, 3);
            cells.iter().for_each(|(Pos(x, y), player)| {
                grid.get_mut((x + 1) as usize, (y + 1) as usize)
                    .map(|p| *p = player.clone());
            });
            grid
        };

        let w = has_winner(grid);
        if matches!(w, Winner::Some(_) | Winner::Draw) {
            state.set(GameState::Won);
            show_text(w, commands, asset_server)
        }
        // match has_winner(grid) {
        //     Winner::Some(p) => {
        //         state.set(GameState::Won);
        //         show_text(Winner::Some(p), commands, font)
        //     }
        //     Winner::Draw => {
        //         state.set(GameState::Won);
        //         show_text(Winner::Draw, commands, font);
        //     }
        //     _ => (),
        // };
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

fn drain_clicks(mut clicks: ResMut<Events<MouseButtonInput>>) {
    clicks.drain();
}

fn reset_onclick(
    mut clicks: EventReader<MouseButtonInput>,
    mut textures: Query<&mut Visible, With<Marker>>,
    mut cells: Query<&mut Option<Player>, With<GridCell>>,
    mut state: ResMut<State<GameState>>,
) {
    if let Some(_) = clicks
        .iter()
        .find(|x| x.button == MouseButton::Left && x.state == ElementState::Pressed)
    {
        textures.iter_mut().for_each(|mut tex| {
            tex.is_visible = false;
        });
        cells.iter_mut().for_each(|mut owner| *owner = None);
        state.set(GameState::Playing);
    }
}

fn show_text(winner: Winner, mut commands: Commands, mut assets: ResMut<AssetServer>) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            match winner {
                Winner::Some(player) => format!("Winner is {:?}! ", player),
                Winner::Draw => format!("No winner. "),
                _ => unreachable!(),
            },
            TextStyle {
                font: assets.load("/usr/share/fonts/TTF/DejaVuSans.ttf"),
                font_size: 20.0,
                color: Default::default(),
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        transform: Transform::from_translation(Vec2::new(0.0, 320.0).extend(0.0)),
        ..Default::default()
    });
}

pub struct GameLogic;
impl Plugin for GameLogic {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Playing)
            .insert_resource(Turn(Player::Red))
            .insert_resource(Winner::None)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(click_gridcell.system().chain(check_winner.system())),
            )
            .add_system_set(SystemSet::on_enter(GameState::Won).with_system(drain_clicks.system()))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(drain_clicks.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Won).with_system(reset_onclick.system()),
            );
    }
}
