use bevy::prelude::*;
use bevy_svg::prelude::*;

mod gridcell;
mod logic;
mod system;

pub use system::CursorPosition;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Player {
    Red,
    Blue,
}

impl Player {
    fn switch(&mut self) {
        match self {
            Player::Red => *self = Self::Blue,
            Player::Blue => *self = Self::Red,
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SvgPlugin)
        .add_plugin(system::GameSystem)
        .add_plugins(logic::GamePlugins)
        .run()
}
