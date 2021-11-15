use bevy::prelude::*;
use bevy_svg::prelude::SvgBundle;

pub struct AddGridCell {
    transform: Transform,
}

#[derive(Bundle, Default)]
struct GridCellBundle {
    gridcell: GridCell,
    pos: Pos,
    owned_by: Option<Player>,
    transform: Transform,
    global_transform: GlobalTransform,
}

enum Player {
    Red,
    Blue,
}

#[derive(Default)]
struct GridCell;

struct Pos(u8, u8);

struct ClickedBy(Player);
