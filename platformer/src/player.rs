use crate::prelude::*;

pub struct Player;

pub fn player_movement(
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

pub fn spawn_player(mut commands: Commands, conf: Res<RapierConfiguration>) {
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
