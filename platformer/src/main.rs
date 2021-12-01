#![allow(non_upper_case_globals)]

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_prototype_lyon::prelude::*;
    pub use bevy_rapier2d::prelude::*;
}
use prelude::*;

mod player;
use player::*;

mod world;
use world::*;

struct UpstreamPlugins;
impl Plugin for UpstreamPlugins {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(WindowDescriptor {
                title: format!("win0"),
                ..Default::default()
            }).add_plugins(DefaultPlugins)
            .add_plugin(ShapePlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

fn main() {
    App::build()
        .add_plugin(UpstreamPlugins)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -25.0).into(),
            scale: 10.0,
            ..Default::default()
        })
        .add_startup_system(spawn_scene.system())
        .add_startup_system(spawn_player.system())
        .add_system(player_movement.system())
        .add_system(move_camera.system())
        .run();
}

