use crate::prelude::*;

pub struct MainCamera;
pub struct Platform;

pub fn move_camera(
    mut transforms: QuerySet<(
        Query<&mut Transform, With<MainCamera>>,
        Query<&Transform, With<crate::player::PlayerEntity>>,
    )>,
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

pub fn spawn_scene(mut commands: Commands, conf: Res<RapierConfiguration>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    const floor_width: f32 = 2000.0;
    const floor_height: f32 = 5.0;
    spawn_platform(&mut commands, conf.scale, Size::new(floor_width, floor_height), Vec3::new(0.0, 5.0, 0.0));
}

fn spawn_platform(commands: &mut Commands, scale: f32, size: Size<f32>, pos: Vec3) {
    commands
        .spawn()
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(size.width / 2.0, size.height / 2.0),
            material: ColliderMaterial {
                friction: 0.9,
                ..Default::default()
            },
            position: ColliderPosition(Isometry::translation(pos.x, pos.y)),
            ..Default::default()
        })
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                width: size.width * scale,
                height: size.height * scale,
                origin: shapes::RectangleOrigin::Center,
            },
            ShapeColors::new(Color::ORANGE_RED),
            DrawMode::Fill(Default::default()),
            Transform::default(),
        ))
        .insert(Platform)
        .insert(ColliderPositionSync::Discrete);
}
