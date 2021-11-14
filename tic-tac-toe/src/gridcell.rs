use bevy::prelude::*;

#[derive(Bundle)]
struct GridCellBundle {
    gridcell: GridCell,
    pos: Pos,
    owned_by: Option<Player>,
}

enum Player {
    Red,
    Blue,
}

struct GridCell;

struct Pos(u8, u8);

struct ClickedBy(Player);
