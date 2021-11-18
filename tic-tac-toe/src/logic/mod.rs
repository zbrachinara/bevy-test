mod in_game;
mod init;

mod prelude {
    pub use bevy::prelude::*;
    pub use crate::gridcell::{self, *};
    pub use super::MainCamera;
}

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct MainCamera;

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(init::GameInit).add(in_game::GameLogic);
    }
}
