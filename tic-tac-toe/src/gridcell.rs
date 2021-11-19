use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_svg::prelude::*;
use std::path::Path;
use crate::Player;

const SIZE: f32 = 200.0;

// #[inline]
pub fn coord_to_pos(mut coord: Vec2) -> Pos {
    coord += Vec2::new(SIZE / 2.0, SIZE / 2.0);

    Pos(
        (coord.x / SIZE).floor() as i8,
        (coord.y / SIZE).floor() as i8,
    )
}

pub struct AddGridCell<P>
where
    P: AsRef<Path> + Send + Sync + 'static,
{
    pub pos: Pos,
    pub red: P,
    pub blue: P,
    pub bbox: P,
}

impl<P: AsRef<Path> + Send + Sync + 'static> Command for AddGridCell<P> {
    fn write(self: Box<Self>, world: &mut World) {
        let transform = Vec2::new(self.pos.0 as f32 * SIZE, self.pos.1 as f32 * SIZE);

        let build = |world: &mut World, path: P| {
            world
                .spawn()
                .insert_bundle(
                    SvgBuilder::from_file(path)
                        .origin(Origin::Center)
                        .scale(Vec2::new(SIZE / 200.0, SIZE / 200.0))
                        .build()
                        .unwrap(),
                )
                .id()
        };
        let build_marker = |world: &mut World, path: P| {
            world
                .spawn()
                .insert_bundle(
                    SvgBuilder::from_file(path)
                        .origin(Origin::Center)
                        .scale(Vec2::new(SIZE / 200.0, SIZE / 200.0))
                        .build()
                        .unwrap(),
                )
                .insert(Marker)
                .id()
        };

        let red = build_marker(world, self.red);
        let blue = build_marker(world, self.blue);
        let bbox = build(world, self.bbox);

        let cell = world
            .spawn()
            .insert_bundle(GridCellBundle {
                pos: self.pos,
                transform: Transform::from_translation(transform.extend(0.0)),
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

#[derive(Default)]
pub struct GridCell;
pub struct Marker;

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos(pub i8, pub i8);
