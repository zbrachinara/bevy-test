mod in_game;
mod init;

mod prelude {
    pub use super::GameState;
    pub use crate::gridcell::*;
    pub use bevy::prelude::*;
    pub use crate::*;
}

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    Won,
}

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(init::GameInit).add(in_game::GameLogic);
    }
}
