use bevy::{platform::collections::HashSet, prelude::*};

use crate::config::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Ground,
    Tree,
    Stockpile,
}

#[derive(Component)]
pub struct TileSprite;

#[derive(Resource)]
pub struct TileEntities {
    pub entities: Vec<Entity>,
}

#[derive(Resource)]
pub struct WorldTrees(pub HashSet<IVec2>);

#[derive(Resource)]
pub struct WorldMap {
    pub tiles: Vec<Tile>,
}

pub fn build_world() -> WorldMap {
    let tiles = vec![Tile::Ground; (MAP_W * MAP_H) as usize];
    let mut world = WorldMap { tiles };

    for y in 0..48 {
        for x in 0..18 {
            set(&mut world, x, y, Tile::Tree);
        }
    }

    for y in 0..18 {
        for x in 0..58 {
            set(&mut world, x, y, Tile::Tree);
        }
    }

    for y in 46..64 {
        for x in 0..58 {
            set(&mut world, x, y, Tile::Tree);
        }
    }

    set(&mut world, STOCKPILE_X, STOCKPILE_Y, Tile::Stockpile);

    world
}

pub fn spawn_world_tiles(commands: &mut Commands, world: &WorldMap) {
    let mut tile_entities = Vec::with_capacity((MAP_W * MAP_H) as usize);
    let mut world_trees = HashSet::with_capacity((MAP_W * MAP_H) as usize);

    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let tile = get(world, x, y);
            if tile == Tile::Tree {
                world_trees.insert(IVec2::new(x, y));
            }

            let color = tile_color(tile);

            let world_pos = grid_to_world(x, y);

            let tile_entity = commands
                .spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::splat(TILE_SIZE - TILE_GAP)),
                        ..default()
                    },
                    Transform::from_translation(world_pos),
                    TileSprite,
                ))
                .id();

            tile_entities.push(tile_entity);
        }
    }

    commands.insert_resource(TileEntities {
        entities: tile_entities,
    });

    commands.insert_resource(WorldTrees(world_trees));
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

pub fn get(map: &WorldMap, x: i32, y: i32) -> Tile {
    map.tiles[idx(x, y)]
}

pub fn set(map: &mut WorldMap, x: i32, y: i32, tile: Tile) {
    map.tiles[idx(x, y)] = tile;
}

pub fn set_with_sprite(
    map: &mut WorldMap,
    tiles: &TileEntities,
    q_tiles: &mut Query<&mut Sprite, With<TileSprite>>,
    x: i32,
    y: i32,
    tile: Tile,
) {
    set(map, x, y, tile);
    let e = tile_entity(tiles, x, y);
    if let Ok(mut sprite) = q_tiles.get_mut(e) {
        sprite.color = tile_color(tile);
    }
}

pub fn tile_entity(tiles: &TileEntities, x: i32, y: i32) -> Entity {
    tiles.entities[idx(x, y)]
}

pub fn tile_color(tile: Tile) -> Color {
    match tile {
        Tile::Ground => Color::srgb(0.15, 0.15, 0.15),
        Tile::Tree => Color::srgb(0.10, 0.35, 0.12),
        Tile::Stockpile => Color::srgb(0.55, 0.42, 0.15),
    }
}
