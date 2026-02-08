use bevy::asset::RenderAssetUsages;
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::config::*;
use crate::world::{self, Tile, WorldMap};

#[derive(Component)]
pub struct Pawn {
    pub id: u32,
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum Task {
    Idle,
    GoToTree(IVec2),
    Chop { at: IVec2, progress: u8 },
    GoToStockpile,
    DropOff,
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Inventory {
    pub wood: u32,
}

pub fn spawn_pawns(commands: &mut Commands, images: &mut ResMut<Assets<Image>>, map: &WorldMap) {
    let circle_image = images.add(make_circle_image(PAWN_RADIUS_PX));

    let max_radius = ((PAWN_COUNT as f32).sqrt().ceil() as i32) + 5;
    let stockpile = IVec2 { x: STOCKPILE_X, y: STOCKPILE_Y };

    let mut occupied: HashSet<IVec2> = HashSet::new();
    let mut spawned = 0usize;

    for p in spiral_positions(stockpile, max_radius) {
        if spawned >= PAWN_COUNT {
            break;
        }

        // bounds check
        if p.x < 0 || p.x >= MAP_W || p.y < 0 || p.y >= MAP_H {
            continue;
        }

        // avoid trees/blocked tiles (customize)
        if world::get(&map, p.x, p.y) == Tile::Tree {
            continue;
        }

        // avoid stacking pawns on same tile
        if occupied.contains(&p) {
            continue;
        }
        occupied.insert(p);

        let pos = world::grid_to_world(p.x, p.y);
        let transform = Transform::from_translation(pos + Vec3::new(0.0, 0.0, 1.0));

        commands.spawn((
            Pawn {
                id: spawned as u32,
                x: p.x,
                y: p.y,
            },
            Sprite {
                image: circle_image.clone(),
                color: Color::srgb(0.85, 0.85, 0.95),
                custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                ..default()
            },
            transform,
        ))
        .insert(Task::Idle)
        .insert(Inventory::default());

        spawned += 1;
    }

    crate::sim::init(commands);
}

fn spiral_positions(center: IVec2, max_radius: i32) -> impl Iterator<Item = IVec2> {
    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = -1;

    let side = max_radius * 2 + 1;
    let steps = side * side;

    (0..steps).map(move |_| {
        let pos = center + IVec2::new(x, y);

        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let tmp = dx;
            dx = -dy;
            dy = tmp;
        }
        x += dx;
        y += dy;

        pos
    })
}


fn make_circle_image(radius: u32) -> Image {
    let size = radius * 2 + 2;
    let w = size as usize;
    let h = size as usize;

    let mut data = vec![0u8; w * h * 4];

    let cx = (size as f32) / 2.0;
    let cy = (size as f32) / 2.0;
    let r = radius as f32;

    for y in 0..h {
        for x in 0..w {
            let fx = x as f32 + 0.5;
            let fy = y as f32 + 0.5;

            let dx = fx - cx;
            let dy = fy - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            let inside = dist <= r;

            let idx = (y * w + x) * 4;
            data[idx + 0] = 255;
            data[idx + 1] = 255;
            data[idx + 2] = 255;
            data[idx + 3] = if inside { 255 } else { 0 };
        }
    }

    Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}
