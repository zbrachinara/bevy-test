use bevy::prelude::*;

#[derive(Bundle)]
struct GridCellBundle {
    gridcell: GridCell,
    pos: Pos,
    owned_by: Option<Player>,
    sprites: GridCellSprites,
}

struct GridCellSprites {
    #[bundle]
    cell_sprite: SpriteBundle,
    #[bundle]
    red_sprite: SpriteBundle,
    #[bundle]
    blue_sprite: SpriteBundle,
}

enum Player {
    Red,
    Blue,
}

struct GridCell;

struct Pos(u8, u8);

struct ClickedBy(Player);
