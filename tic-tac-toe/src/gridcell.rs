use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_svg::prelude::*;
use std::path::Path;

pub struct AddGridCell<P>
where P: AsRef<Path> + Send + Sync + 'static
{
    pub pos: Pos,
    pub red: P,
    pub blue: P,
    pub bbox: P,
}

impl<P: AsRef<Path> + Send + Sync + 'static> Command for AddGridCell<P> {
    fn write(self: Box<Self>, world: &mut World) {
        let transform = Vec3::new(
            self.pos.0 as f32 * 200f32,
            self.pos.1 as f32 as f32 * 200f32,
            0.0,
        );

        let red = world
            .spawn()
            .insert_bundle(
                SvgBuilder::from_file(self.red)
                    .build()
                    .unwrap(),
            )
            .id();
        let blue = world
            .spawn()
            .insert_bundle(
                SvgBuilder::from_file(self.blue)
                    .build()
                    .unwrap(),
            )
            .id();
        let bbox = world
            .spawn()
            .insert_bundle(
                SvgBuilder::from_file(self.bbox)
                    .build()
                    .unwrap(),
            )
            .id();

        // world
        //     .entity_mut(red.clone())
        //     .get_mut::<Visible>()
        //     .map(|mut visible| {
        //         visible.is_transparent = true;
        //     });

        let cell = world
            .spawn()
            .insert_bundle(GridCellBundle {
                pos: self.pos,
                transform: Transform::from_translation(transform),
                ..Default::default()
            })
            .id();

        world.entity_mut(cell).push_children(&[red, blue, bbox]);
    }
}

#[derive(Bundle, Default)]
struct GridCellBundle {
    gridcell: GridCell,
    pos: Pos,
    owned_by: Option<Player>,
    transform: Transform,
    global_transform: GlobalTransform,
}

enum Player {
    Red,
    Blue,
}

#[derive(Default)]
struct GridCell;

#[derive(Default)]
pub struct Pos(pub u8, pub u8);

struct ClickedBy(Player);
