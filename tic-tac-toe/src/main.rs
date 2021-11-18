use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use bevy::prelude::*;
use std::path::{Path, PathBuf};

use crate::gridcell::{coord_to_pos, Pos};
use bevy_svg::prelude::*;

pub mod gridcell;
mod logic;


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SvgPlugin)
        .add_plugins(logic::GamePlugins)
        .run()
}
