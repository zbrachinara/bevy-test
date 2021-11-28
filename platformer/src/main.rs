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
        // .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .run();
}

fn setup_graphics(mut commands: Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 2.8;

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 200.0, 0.0));
    // commands.spawn_bundle(LightBundle {
    //     transform: Transform::from_translation(Vec3::new(1000.0, 10.0, 2000.0)),
    //     light: Light {
    //         intensity: 100_000_000_.0,
    //         range: 6000.0,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
    commands.spawn_bundle(camera);
}

pub fn setup_physics(mut commands: Commands, conf: Res<RapierConfiguration>) {
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
        .insert(ColliderDebugRender::default())
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
        .insert(ColliderDebugRender::with_id(0))
        .insert(ColliderPositionSync::Discrete);
}
