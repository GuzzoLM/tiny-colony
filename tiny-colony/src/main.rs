use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

const MAP_W: i32 = 64;
const MAP_H: i32 = 64;
const TILE_SIZE: f32 = 12.0;
const TILE_GAP: f32 = 1.0;

const STOCKPILE_X: i32 = MAP_W / 2;
const STOCKPILE_Y: i32 = MAP_H / 2;

const PAWN_COUNT: usize = 10;
const PAWN_RADIUS_PX: u32 = 12;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Ground,
    Tree,
    Stockpile,
}

#[derive(Component)]
struct Pawn {
    id: u32,
    x: i32,
    y: i32,
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    let mut tiles = vec![Tile::Ground; (MAP_W * MAP_H) as usize];

    for y in 10..18 {
        for x in 10..18 {
            set_tile(&mut tiles, x, y, Tile::Tree);
        }
    }

    set_tile(&mut tiles, STOCKPILE_X, STOCKPILE_Y, Tile::Stockpile);

    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let tile = get_tile(&tiles, x, y);

            let color = match tile {
                Tile::Ground => Color::srgb(0.15, 0.15, 0.15),
                Tile::Tree => Color::srgb(0.10, 0.35, 0.12),
                Tile::Stockpile => Color::srgb(0.55, 0.42, 0.15),
            };

            let world_pos = grid_to_world(x, y);

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE - TILE_GAP)),
                    ..default()
                },
                Transform::from_translation(world_pos),
            ));
        }
    }

    let circle_image = images.add(make_circle_image(PAWN_RADIUS_PX));

    let spawn_offsets: [(i32, i32); PAWN_COUNT] = [
        (1, 0), (-1, 0), (0, 1), (0, -1), (1, 1),
        (-1, 1), (1, -1), (-1, -1), (2, 0), (-2, 0)
    ];

    for (i, (dx, dy)) in spawn_offsets.into_iter().enumerate() {
        let x = STOCKPILE_X + dx;
        let y = STOCKPILE_Y + dy;

        let pos = grid_to_world(x, y);
        let transform = Transform::from_translation(pos + Vec3::new(0.0, 0.0, 1.0));

        commands.spawn((
            Pawn { id: i as u32, x, y },
            Sprite {
                image: circle_image.clone(),
                color: Color::srgb(0.85, 0.85, 0.95),
                custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                ..default()
            },
            transform,
        ));
    }
}


fn grid_to_world(x: i32, y: i32) -> Vec3 {
    let origin_x = -(MAP_W as f32) * TILE_SIZE * 0.5 + TILE_SIZE * 0.5;
    let origin_y = -(MAP_H as f32) * TILE_SIZE * 0.5 + TILE_SIZE * 0.5;

    Vec3::new(
        origin_x + x as f32 * TILE_SIZE,
        origin_y + y as f32 * TILE_SIZE,
        0.0,
    )
}

fn idx(x: i32, y: i32) -> usize {
    (y * MAP_W + x) as usize
}

fn set_tile(tiles: &mut [Tile], x: i32, y: i32, tile: Tile) {
    tiles[idx(x, y)] = tile;
}

fn get_tile(tiles: &[Tile], x: i32, y: i32) -> Tile {
    tiles[idx(x, y)]
}

fn make_circle_image(radius: u32) -> Image {
    let size = radius * 2 + 2; // small padding
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
            data[idx + 0] = 255; // white (we tint with Sprite.color)
            data[idx + 1] = 255;
            data[idx + 2] = 255;
            data[idx + 3] = if inside { 255 } else { 0 }; // alpha mask
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
