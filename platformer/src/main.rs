use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::render::pass::ClearColor;
use bevy_prototype_lyon::prelude::*;
use nalgebra::Isometry2;

fn main() {
    App::build()
        // .insert_resource(ClearColor(Color::rgb(
        //     0xF9 as f32 / 255.0,
        //     0xF9 as f32 / 255.0,
        //     0xFF as f32 / 255.0,
        // )))
        .insert_resource(WindowDescriptor {
            title: format!("win0"),
            ..Default::default()
        })
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .run();
}

fn setup_graphics(mut commands: Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 10.0;

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 200.0, 0.0));
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 10.0, 2000.0)),
        light: Light {
            intensity: 100_000_000_.0,
            range: 6000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(camera);
}

pub fn setup_physics(mut commands: Commands) {
    //platform
    let floor = commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1000.0, 1.0),
            ..Default::default()
        })
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                width: 1000.0,
                height: 1.0,
                origin: shapes::RectangleOrigin::Center,
            },
            ShapeColors::new(Color::ORANGE_RED),
            DrawMode::Fill(Default::default()),
            Transform::default(),
        ))
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete);

    //cube
    commands
        .spawn_bundle(RigidBodyBundle {
            position: [0.0, 10.0].into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 1.0),
            ..Default::default()
        })
        .insert(ColliderDebugRender::with_id(0))
        .insert(ColliderPositionSync::Discrete);
}
