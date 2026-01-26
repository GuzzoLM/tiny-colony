use bevy::prelude::*;

use crate::config::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Ground,
    Tree,
    Stockpile,
}

#[derive(Resource)]
pub struct WorldMap {
    pub tiles: Vec<Tile>,
}

pub fn build_world() -> WorldMap {
    let tiles = vec![Tile::Ground; (MAP_W * MAP_H) as usize];
    let mut world = WorldMap { tiles };

    for y in 10..18 {
        for x in 10..18 {
            set(&mut world, x, y, Tile::Tree);
        }
    }

    set(&mut world, STOCKPILE_X, STOCKPILE_Y, Tile::Stockpile);

    world
}

pub fn spawn_world_tiles(commands: &mut Commands, world: &WorldMap) {
    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let tile = get(world, x, y);

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

pub fn in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < MAP_W && y >= 0 && y < MAP_H
}

pub fn get(map: &WorldMap, x: i32, y: i32) -> Tile {
    map.tiles[idx(x, y)]
}

pub fn set(map: &mut WorldMap, x: i32, y: i32, tile: Tile) {
    map.tiles[idx(x, y)] = tile;
}
