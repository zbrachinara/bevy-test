use bevy::prelude::*;
use crate::MainCamera;

fn mouse(windows: Res<Windows>, camera: Query<&Transform, With<MainCamera>>) -> Option<Vec2> {
    let window = windows.get_primary().unwrap();
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
    }
}