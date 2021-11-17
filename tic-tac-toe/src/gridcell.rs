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

        let build = |world: &mut World, path: P| {
            world
                .spawn()
                .insert_bundle(SvgBuilder::from_file(path).build().unwrap())
                .id()
        };

        let red = build(world, self.red);
        let blue = build(world, self.blue);
        let bbox = build(world, self.bbox);

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
