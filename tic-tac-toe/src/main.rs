use bevy::prelude::*;
use bevy_svg::prelude::*;

mod gridcell;
mod logic;
mod system;

pub struct MainCamera;
pub use system::CursorPosition;

pub enum Player {
    Red,
    Blue,
}

fn make_ui(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SvgPlugin)
        .add_plugin(system::GameSystem)
        .add_startup_system(make_ui.system())
        .add_plugins(logic::GamePlugins)
        .run()
}
