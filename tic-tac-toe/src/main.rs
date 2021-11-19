use bevy::prelude::*;
use bevy_svg::prelude::*;

mod gridcell;
mod logic;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SvgPlugin)
        .add_plugins(logic::GamePlugins)
        .run()
}
