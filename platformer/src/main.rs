#![allow(non_upper_case_globals)]

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

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
            gravity: Vec2::new(0.0, -25.0).into(),
            scale: 10.0,
            ..Default::default()
        })
        .add_startup_system(spawn_objects.system())
        .add_system(player_movement.system())
        .add_system(move_camera.system())
        .run();
}

fn move_camera(
    mut transforms: QuerySet<(
        Query<&mut Transform, With<MainCamera>>,
        Query<&Transform, With<Player>>,
    )>,
    // mut velocity: Query<&mut RigidBodyVelocity, With<MainCamera>>,
) {
    const step: f32 = 0.01;

    let player_transform = transforms.q1().single().ok().map(|borrow| borrow.clone());
    let cam_transform = transforms.q0_mut().single_mut().ok();

    if let Some((mut cam_transform, player_transform)) = cam_transform.zip(player_transform) {
        let diff = player_transform.translation - cam_transform.translation;
        if Vec3::abs(diff) > Vec2::splat(20.0).extend(0.0) {
            cam_transform.translation += diff * step;
        }
    }
}

fn player_movement(
    mut player: Query<&mut RigidBodyVelocity, With<Player>>,
    key: Res<Input<KeyCode>>,
) {
    if let Ok(mut vel) = player.single_mut() {
        const lateral_power: f32 = 1.2;
        const max_lateral_power: f32 = 20.0;

        if vel.linvel.x.abs() <= max_lateral_power {
            if key.pressed(KeyCode::D) {
                vel.linvel.x += lateral_power;
            }
            if key.pressed(KeyCode::A) {
                vel.linvel.x -= lateral_power;
            }
        }
    }
}

struct Player;
struct MainCamera;

fn spawn_objects(mut commands: Commands, conf: Res<RapierConfiguration>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    const floor_width: f32 = 2000.0;
    const floor_height: f32 = 5.0;
    //platform
    let floor = commands
        .spawn()
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(floor_width / 2.0, floor_height / 2.0),
            material: ColliderMaterial {
                friction: 0.7,
                ..Default::default()
            },
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
    const cube_size: f32 = 5.5;
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            position: [0.0, 10.5].into(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            },
            mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
            damping: RigidBodyDamping {
                linear_damping: 0.99,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(cube_size / 2.0, cube_size / 2.0),
            flags: (ActiveEvents::CONTACT_EVENTS).into(),
            ..Default::default()
        })
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
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
