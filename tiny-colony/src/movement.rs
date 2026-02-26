use std::collections::VecDeque;

use bevy::prelude::*;

use crate::config::{MAP_H, MAP_W};
use crate::pawn::Pawn;
use crate::world::{self, Tile, WorldMap};

pub const MAX_WAIT_TICKS: u8 = 5;
const BASE_REPATH_TICKS: u16 = 10;
const MAX_REPATH_TICKS: u16 = 80;
const STUCK_TICKS: u16 = 30;

#[derive(Resource)]
pub struct OccupancyGrid {
    tiles: Vec<Option<Entity>>,
}

impl OccupancyGrid {
    pub fn new() -> Self {
        Self {
            tiles: vec![None; (MAP_W * MAP_H) as usize],
        }
    }

    pub fn is_free(&self, pos: IVec2) -> bool {
        if !in_bounds(pos) {
            return false;
        }
        self.get(pos).is_none()
    }

    pub fn get(&self, pos: IVec2) -> Option<Entity> {
        if !in_bounds(pos) {
            return None;
        }
        self.tiles[idx(pos)]
    }

    pub fn set(&mut self, pos: IVec2, entity: Option<Entity>) {
        if !in_bounds(pos) {
            return;
        }
        let i = idx(pos);
        self.tiles[i] = entity;
    }

    pub fn move_entity(&mut self, from: IVec2, to: IVec2, entity: Entity) {
        self.set(from, None);
        self.set(to, Some(entity));
    }
}

#[derive(Component, Debug, Clone)]
pub struct Movement {
    pub path: VecDeque<IVec2>,
    pub blocked_ticks: u8,
    pub repath_cooldown: u16,
    pub repath_backoff: u16,
    pub no_move_ticks: u16,
    pub last_pos: IVec2,
}

impl Movement {
    pub fn new(pos: IVec2) -> Self {
        Self {
            path: VecDeque::new(),
            blocked_ticks: 0,
            repath_cooldown: 0,
            repath_backoff: BASE_REPATH_TICKS,
            no_move_ticks: 0,
            last_pos: pos,
        }
    }

    pub fn clear_path(&mut self) {
        self.path.clear();
        self.blocked_ticks = 0;
    }

    fn queue_repath(&mut self) {
        if self.repath_backoff == 0 {
            self.repath_backoff = BASE_REPATH_TICKS;
        }
        self.repath_cooldown = self.repath_backoff;
        self.repath_backoff = (self.repath_backoff * 2).min(MAX_REPATH_TICKS);
    }

    fn record_move(&mut self, pos: IVec2) {
        self.no_move_ticks = 0;
        self.last_pos = pos;
    }

    fn record_no_move(&mut self) -> bool {
        self.no_move_ticks = self.no_move_ticks.saturating_add(1);
        self.no_move_ticks >= STUCK_TICKS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveOutcome {
    Arrived,
    Moving,
    Waiting,
    Repathing,
    Stuck,
    NoPath,
}

pub fn move_towards_target(
    pawn_entity: Entity,
    pawn: &mut Pawn,
    movement: &mut Movement,
    transform: &mut Transform,
    target: IVec2,
    map: &WorldMap,
    occupancy: &mut OccupancyGrid,
) -> MoveOutcome {
    let current = IVec2::new(pawn.x, pawn.y);
    if current == target {
        movement.clear_path();
        movement.record_move(current);
        return MoveOutcome::Arrived;
    }

    if movement.repath_cooldown > 0 {
        movement.repath_cooldown -= 1;
        let stuck = movement.record_no_move();
        return if stuck { MoveOutcome::Stuck } else { MoveOutcome::Waiting };
    }

    if movement.path.is_empty() {
        if let Some(path) = plan_path(current, target, map, occupancy) {
            movement.path = path;
        } else {
            movement.queue_repath();
            let stuck = movement.record_no_move();
            return if stuck { MoveOutcome::Stuck } else { MoveOutcome::NoPath };
        }
    }

    if movement.path.is_empty() {
        movement.queue_repath();
        let stuck = movement.record_no_move();
        return if stuck { MoveOutcome::Stuck } else { MoveOutcome::NoPath };
    }

    let next = match movement.path.front().copied() {
        Some(next) => next,
        None => {
            movement.queue_repath();
            let stuck = movement.record_no_move();
            return if stuck { MoveOutcome::Stuck } else { MoveOutcome::NoPath };
        }
    };

    if !is_adjacent(current, next) {
        movement.clear_path();
        movement.queue_repath();
        let stuck = movement.record_no_move();
        return if stuck { MoveOutcome::Stuck } else { MoveOutcome::Repathing };
    }
    if !is_walkable(next, target, map) {
        movement.clear_path();
        movement.queue_repath();
        let stuck = movement.record_no_move();
        return if stuck { MoveOutcome::Stuck } else { MoveOutcome::Repathing };
    }

    if occupancy.is_free(next) || occupancy.get(next) == Some(pawn_entity) {
        occupancy.move_entity(current, next, pawn_entity);
        pawn.x = next.x;
        pawn.y = next.y;
        update_transform(transform, pawn);
        movement.path.pop_front();
        movement.blocked_ticks = 0;
        movement.repath_backoff = BASE_REPATH_TICKS;
        movement.record_move(next);

        if next == target && movement.path.is_empty() {
            return MoveOutcome::Arrived;
        }
        return MoveOutcome::Moving;
    }

    movement.blocked_ticks = movement.blocked_ticks.saturating_add(1);
    if movement.blocked_ticks <= MAX_WAIT_TICKS {
        let stuck = movement.record_no_move();
        return if stuck { MoveOutcome::Stuck } else { MoveOutcome::Waiting };
    }

    if let Some(side_step) = find_side_step(current, target, map, occupancy) {
        occupancy.move_entity(current, side_step, pawn_entity);
        pawn.x = side_step.x;
        pawn.y = side_step.y;
        update_transform(transform, pawn);
        movement.blocked_ticks = 0;
        movement.record_move(side_step);
        return MoveOutcome::Moving;
    }

    movement.clear_path();
    movement.queue_repath();
    let stuck = movement.record_no_move();
    if stuck {
        MoveOutcome::Stuck
    } else {
        MoveOutcome::Repathing
    }
}

fn update_transform(transform: &mut Transform, pawn: &Pawn) {
    let pos = world::grid_to_world(pawn.x, pawn.y);
    transform.translation = pos + Vec3::new(0.0, 0.0, 1.0);
}

fn plan_path(
    start: IVec2,
    goal: IVec2,
    map: &WorldMap,
    occupancy: &OccupancyGrid,
) -> Option<VecDeque<IVec2>> {
    if start == goal {
        return Some(VecDeque::new());
    }

    let mut queue = VecDeque::new();
    let mut came_from: Vec<Option<IVec2>> = vec![None; (MAP_W * MAP_H) as usize];

    queue.push_back(start);
    came_from[idx(start)] = Some(start);

    while let Some(current) = queue.pop_front() {
        for neighbor in neighbors(current) {
            if !in_bounds(neighbor) {
                continue;
            }
            let n_idx = idx(neighbor);
            if came_from[n_idx].is_some() {
                continue;
            }
            if !is_walkable(neighbor, goal, map) {
                continue;
            }
            if neighbor != goal && !occupancy.is_free(neighbor) {
                continue;
            }

            came_from[n_idx] = Some(current);
            if neighbor == goal {
                queue.clear();
                break;
            }
            queue.push_back(neighbor);
        }
    }

    if came_from[idx(goal)].is_none() {
        return None;
    }

    let mut path = Vec::new();
    let mut current = goal;
    while current != start {
        path.push(current);
        current = came_from[idx(current)]?;
    }
    path.reverse();
    Some(VecDeque::from(path))
}

fn neighbors(pos: IVec2) -> [IVec2; 4] {
    [
        IVec2::new(pos.x + 1, pos.y),
        IVec2::new(pos.x - 1, pos.y),
        IVec2::new(pos.x, pos.y + 1),
        IVec2::new(pos.x, pos.y - 1),
    ]
}

fn is_adjacent(a: IVec2, b: IVec2) -> bool {
    (a.x - b.x).abs() + (a.y - b.y).abs() == 1
}

fn find_side_step(
    current: IVec2,
    target: IVec2,
    map: &WorldMap,
    occupancy: &OccupancyGrid,
) -> Option<IVec2> {
    for candidate in neighbors(current) {
        if !in_bounds(candidate) {
            continue;
        }
        if !is_walkable(candidate, target, map) {
            continue;
        }
        if occupancy.is_free(candidate) {
            return Some(candidate);
        }
    }
    None
}

fn is_walkable(pos: IVec2, target: IVec2, map: &WorldMap) -> bool {
    if pos == target {
        return true;
    }

    match world::get(map, pos.x, pos.y) {
        Tile::Ground | Tile::Stockpile => true,
        Tile::Tree => false,
    }
}

fn in_bounds(pos: IVec2) -> bool {
    pos.x >= 0 && pos.x < MAP_W && pos.y >= 0 && pos.y < MAP_H
}

fn idx(pos: IVec2) -> usize {
    (pos.y * MAP_W + pos.x) as usize
}
