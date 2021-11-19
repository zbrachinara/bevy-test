mod in_game;
mod init;

mod prelude {
    pub use super::MainCamera;
    pub use crate::gridcell::*;
    pub use bevy::prelude::*;
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
