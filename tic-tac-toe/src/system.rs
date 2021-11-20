use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct CursorPosition(Option<Vec2>);

impl Deref for CursorPosition {
    type Target = Option<Vec2>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CursorPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn mouse(
    windows: Res<Windows>,
    camera: Query<&Transform, With<MainCamera>>,
    mut position_res: ResMut<CursorPosition>,
) {
    let window = windows.get_primary().unwrap();
    let position =
        if let Some((pos, camera_transform)) = window.cursor_position().zip(camera.single().ok()) {
            let size = Vec2::new(window.width(), window.height());
            let pos = (pos - size / 2.0)
                // extend to vec4
                .extend(0.0)
                .extend(1.0);
            let world_pos = camera_transform.compute_matrix() * pos;

            Some(Vec2::new(world_pos.x, world_pos.y))
        } else {
            None
        };

    **position_res = position;
}

pub struct MainCamera;
fn make_ui(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

pub struct GameSystem;
impl Plugin for GameSystem {
    fn build(&self, app: &mut AppBuilder) {
        static CALCULATION_STAGE: &str = "CALCULATION_STAGE";
        app.init_resource::<CursorPosition>()
            .add_stage_before(
                CoreStage::Update,
                CALCULATION_STAGE,
                SystemStage::parallel(),
            )
            .add_system_to_stage(CALCULATION_STAGE, mouse.system())
            .add_startup_system(make_ui.system());
    }
}
