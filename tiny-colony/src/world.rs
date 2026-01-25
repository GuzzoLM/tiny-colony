use bevy::prelude::*;

use crate::config::*;

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Ground,
    Tree,
    Stockpile,
}

pub fn build_world() -> Vec<Tile> {
    let mut tiles = vec![Tile::Ground; (MAP_W * MAP_H) as usize];

    for y in 10..18 {
        for x in 10..18 {
            set_tile(&mut tiles, x, y, Tile::Tree);
        }
    }

    set_tile(&mut tiles, STOCKPILE_X, STOCKPILE_Y, Tile::Stockpile);

    tiles
}

pub fn spawn_world_tiles(commands: &mut Commands, tiles: &[Tile]) {
    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let tile = get_tile(tiles, x, y);

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
}

pub fn grid_to_world(x: i32, y: i32) -> Vec3 {
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
