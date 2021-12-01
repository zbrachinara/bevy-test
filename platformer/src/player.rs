use crate::prelude::*;
use crate::Platform;
use derive_more::{Deref, DerefMut};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player_movement.system())
            .add_system(is_player_colliding.system())
            .add_startup_system(spawn_player.system())
            .insert_resource(IsPlayerContacting(false));
    }
}

pub struct PlayerEntity;
#[derive(Deref, DerefMut, Debug)]
pub struct IsPlayerContacting(pub bool);

pub fn is_player_colliding(
    player_handle: Query<Entity, With<PlayerEntity>>,
    platform_handles: Query<Entity, With<Platform>>,
    collision_checker: Res<NarrowPhase>,
    mut out: ResMut<IsPlayerContacting>,
) {
    if let Ok(player) = player_handle.single() {
        **out = platform_handles.iter().any(|collider| {
            match collision_checker.contact_pair(player.handle(), collider.handle()) {
                Some(pair) => pair.has_any_active_contact,
                None => false,
            }
        });
        // println!("Is colliding? {}", colliding);
    };
}

pub fn player_movement(
    mut player: Query<&mut RigidBodyVelocity, With<PlayerEntity>>,
    key: Res<Input<KeyCode>>,
    is_contacting: Res<IsPlayerContacting>,
) {
    if let Ok(mut vel) = player.single_mut() {
        const lateral_power: f32 = 1.2;
        const max_lateral_power: f32 = 20.0;

        if **is_contacting {
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
        .insert(PlayerEntity)
        .insert(ColliderPositionSync::Discrete);
}
