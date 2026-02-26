use bevy::prelude::*;

use crate::colony::Colony;
use crate::config::*;
use crate::movement::{move_towards_target, MoveOutcome, Movement, OccupancyGrid};
use crate::pawn::{Inventory, Pawn, Task};
use crate::sim::Reservations;
use crate::world::{self, Tile, WorldMap, WorldTrees};

pub fn handle_idle(
    pawn_entity: Entity,
    pawn: &Pawn,
    map: &WorldMap,
    reservations: &mut Reservations,
    world_trees: &WorldTrees,
) -> Task {
    if let Some(tree) =
        find_nearest_tree(map, IVec2::new(pawn.x, pawn.y), reservations, world_trees)
    {
        reservations.reserved_tiles.insert(tree, pawn_entity);
        Task::GoToTree(tree)
    } else {
        Task::Idle
    }
}

pub fn handle_go_to_tree(
    pawn_entity: Entity,
    pawn: &mut Pawn,
    movement: &mut Movement,
    transform: &mut Transform,
    map: &WorldMap,
    occupancy: &mut OccupancyGrid,
    reservations: &mut Reservations,
    at: IVec2,
) -> Task {
    if world::get(map, at.x, at.y) != Tile::Tree {
        if reservations.reserved_tiles.get(&at) == Some(&pawn_entity) {
            reservations.reserved_tiles.remove(&at);
        }
        movement.clear_path();
        return Task::Idle;
    }

    match move_towards_target(
        pawn_entity,
        pawn,
        movement,
        transform,
        at,
        map,
        occupancy,
    ) {
        MoveOutcome::Arrived => Task::Chop { at, progress: 0 },
        MoveOutcome::Stuck => {
            movement.clear_path();
            if reservations.reserved_tiles.get(&at) == Some(&pawn_entity) {
                reservations.reserved_tiles.remove(&at);
            }
            Task::Idle
        }
        _ => Task::GoToTree(at),
    }
}

pub fn handle_chop(
    pawn_entity: Entity,
    map: &mut WorldMap,
    inv: &mut Inventory,
    at: IVec2,
    progress: u8,
    reservations: &mut Reservations,
    world_trees: &mut WorldTrees,
    tile_entities: &mut Res<world::TileEntities>,
    q_tiles: &mut Query<&mut Sprite, With<world::TileSprite>>,
) -> Task {
    if world::get(map, at.x, at.y) != Tile::Tree {
        if reservations.reserved_tiles.get(&at) == Some(&pawn_entity) {
            reservations.reserved_tiles.remove(&at);
        }
        return Task::Idle;
    }

    let next = progress + 1;
    if next >= 10 {
        world::set_with_sprite(map, &tile_entities, q_tiles, at.x, at.y, Tile::Ground);
        inv.wood += 1;
        world_trees.0.remove(&at);
        if reservations.reserved_tiles.get(&at) == Some(&pawn_entity) {
            reservations.reserved_tiles.remove(&at);
        }
        Task::GoToStockpile
    } else {
        Task::Chop { at, progress: next }
    }
}

pub fn handle_go_to_stockpile(
    pawn_entity: Entity,
    pawn: &mut Pawn,
    movement: &mut Movement,
    transform: &mut Transform,
    map: &WorldMap,
    occupancy: &mut OccupancyGrid,
) -> Task {
    let target = IVec2::new(STOCKPILE_X, STOCKPILE_Y);
    match move_towards_target(
        pawn_entity,
        pawn,
        movement,
        transform,
        target,
        map,
        occupancy,
    ) {
        MoveOutcome::Arrived => Task::DropOff,
        MoveOutcome::Stuck => {
            movement.clear_path();
            Task::Idle
        }
        _ => Task::GoToStockpile,
    }
}

pub fn handle_drop_off(inv: &mut Inventory, stockpile: &mut Colony) -> Task {
    if inv.wood > 0 {
        stockpile.wood += inv.wood;
        inv.wood = 0;
    }
    Task::Idle
}

fn find_nearest_tree(
    map: &WorldMap,
    from: IVec2,
    reservations: &Reservations,
    world_trees: &WorldTrees,
) -> Option<IVec2> {
    let mut best: Option<(i32, IVec2)> = None;

    for &target in world_trees.0.iter() {
        let reserved = reservations.reserved_tiles.contains_key(&target);
        if !reserved && world::get(map, target.x, target.y) == Tile::Tree {
            let dist = (from.x - target.x).abs() + (from.y - target.y).abs();

            match best {
                None => best = Some((dist, target)),
                Some((best_dist, _)) if dist < best_dist => best = Some((dist, target)),
                _ => {}
            }
        }
    }

    best.map(|(_, pos)| pos)
}
