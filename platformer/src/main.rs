use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::na::{Const, Matrix2, Vector2};
use crate::nalgebra::ArrayStorage;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: format!("win0"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            scale: 2.0,
            ..Default::default()
        })
        .add_startup_system(spawn_objects.system())
        .add_system(input.system())
        .run();
}

fn input(mut player: Query<&mut RigidBodyVelocity, With<Player>>, key: Res<Input<KeyCode>>) {
    if let Ok(mut vel) = player.single_mut() {
        if key.pressed(KeyCode::D) {
            vel.linvel.x += 5.0;
            vel.angvel = 0.0;
        }
        if key.pressed(KeyCode::A) {
            vel.linvel.x -= 5.0;
            vel.angvel = 0.0;
        }
    }
}

struct Player;

fn spawn_objects(mut commands: Commands, conf: Res<RapierConfiguration>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    const floor_width: f32 = 1000.0;
    const floor_height: f32 = 5.0;
    //platform
    let floor = commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(floor_width / 2.0, floor_height / 2.0),
            ..Default::default()
        })
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                width: floor_width * conf.scale,
                height: floor_height * conf.scale,
                origin: shapes::RectangleOrigin::Center,
            },
            ShapeColors::new(Color::ORANGE_RED),
            DrawMode::Fill(Default::default()),
            Transform::default(),
        ))
        .insert(ColliderPositionSync::Discrete);

    //cube
    const cube_size: f32 = 20.0;
    commands
        .spawn_bundle(RigidBodyBundle {
            position: [0.0, 40.0].into(),
            ccd: RigidBodyCcd { ccd_enabled: true, ..Default::default()},
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(cube_size / 2.0, cube_size / 2.0),
            ..Default::default()
        })
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle{
                width: cube_size * conf.scale,
                height: cube_size * conf.scale,
                origin: shapes::RectangleOrigin::Center,
            },
            ShapeColors::new(Color::BLUE),
            DrawMode::Fill(FillOptions::default()),
            Transform::default(),
        ))
        .insert(Player)
        .insert(ColliderPositionSync::Discrete);
}
