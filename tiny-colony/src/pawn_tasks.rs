use bevy::prelude::*;

use crate::colony::Colony;
use crate::config::*;
use crate::pawn::{Inventory, Pawn, Task};
use crate::world::{self, Tile, WorldMap};

pub fn handle_idle(pawn: &Pawn, map: &WorldMap) -> Task {
    if let Some(tree) = find_nearest_tree(map, IVec2::new(pawn.x, pawn.y)) {
        Task::GoToTree(tree)
    } else {
        Task::Idle
    }
}

pub fn handle_go_to_tree(pawn: &mut Pawn, transform: &mut Transform, target: IVec2) -> Task {
    let arrived = move_and_update(pawn, transform, target);
    if arrived {
        Task::Chop { at: target, progress: 0 }
    } else {
        Task::GoToTree(target)
    }
}

pub fn handle_chop(map: &mut WorldMap, inv: &mut Inventory, at: IVec2, progress: u8) -> Task {
    if world::get(map, at.x, at.y) != Tile::Tree {
        return Task::Idle;
    }

    let next = progress + 1;
    if next >= 10 {
        world::set(map, at.x, at.y, Tile::Ground);
        inv.wood += 1;
        Task::GoToStockpile
    } else {
        Task::Chop { at, progress: next }
    }
}

pub fn handle_go_to_stockpile(pawn: &mut Pawn, transform: &mut Transform) -> Task {
    let target = IVec2::new(STOCKPILE_X, STOCKPILE_Y);
    let arrived = move_and_update(pawn, transform, target);
    if arrived {
        Task::DropOff
    } else {
        Task::GoToStockpile
    }
}

pub fn handle_drop_off(inv: &mut Inventory, stockpile: &mut Colony) -> Task {
    if inv.wood > 0 {
        stockpile.wood += inv.wood;
        inv.wood = 0;
    }
    Task::Idle
}

fn move_and_update(pawn: &mut Pawn, transform: &mut Transform, target: IVec2) -> bool {
    step_towards(pawn, target);
    update_transform(transform, pawn);
    pawn.x == target.x && pawn.y == target.y
}

fn update_transform(transform: &mut Transform, pawn: &Pawn) {
    let pos = world::grid_to_world(pawn.x, pawn.y);
    transform.translation = pos + Vec3::new(0.0, 0.0, 1.0);
}

fn step_towards(pawn: &mut Pawn, target: IVec2) {
    if pawn.x < target.x {
        pawn.x += 1;
    } else if pawn.x > target.x {
        pawn.x -= 1;
    } else if pawn.y < target.y {
        pawn.y += 1;
    } else if pawn.y > target.y {
        pawn.y -= 1;
    }
}

fn find_nearest_tree(map: &WorldMap, from: IVec2) -> Option<IVec2> {
    let mut best: Option<(i32, IVec2)> = None;

    for y in 0..MAP_H {
        for x in 0..MAP_W {
            if world::get(map, x, y) == Tile::Tree {
                let dist = (from.x - x).abs() + (from.y - y).abs();
                let pos = IVec2::new(x, y);

                match best {
                    None => best = Some((dist, pos)),
                    Some((best_dist, _)) if dist < best_dist => best = Some((dist, pos)),
                    _ => {}
                }
            }
        }
    }

    best.map(|(_, pos)| pos)
}
