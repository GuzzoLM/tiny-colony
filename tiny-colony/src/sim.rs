use bevy::platform::collections::HashMap;
use bevy::prelude::*;

use crate::colony::Colony;
use crate::pawn::{Inventory, Pawn, Task};
use crate::pawn_tasks;
use crate::world::{self, WorldMap};

#[derive(Resource)]
pub struct Sim {
    pub paused: bool,
    pub speed: f32,
    pub tick: Timer,
}

#[derive(Resource)]
pub struct Reservations {
    pub reserved_tiles: HashMap<IVec2, Entity>
}

pub fn init(commands: &mut Commands) {
    commands.insert_resource(Sim {
        paused: false,
        speed: 1.0,
        tick: Timer::from_seconds(0.10, TimerMode::Repeating), // 10 Hz
    });

    commands.insert_resource(Colony::default());
    commands.insert_resource(Reservations {
        reserved_tiles: HashMap::new(),
    });
}

pub fn sim_controls(keys: Res<ButtonInput<KeyCode>>, mut sim: ResMut<Sim>) {
    if keys.just_pressed(KeyCode::Space) {
        sim.paused = !sim.paused;
    }

    if keys.just_pressed(KeyCode::Digit1) {
        sim.speed = 1.0;
    } else if keys.just_pressed(KeyCode::Digit2) {
        sim.speed = 2.0;
    } else if keys.just_pressed(KeyCode::Digit3) {
        sim.speed = 4.0;
    }
}

pub fn tick_jobs(
    time: Res<Time>,
    mut sim: ResMut<Sim>,
    mut map: ResMut<WorldMap>,
    mut stockpile: ResMut<Colony>,
    mut q: Query<(Entity, &mut Pawn, &mut Transform, &mut Task, &mut Inventory)>,
    mut tile_entities: Res<world::TileEntities>,
    mut q_tiles: Query<&mut Sprite, With<world::TileSprite>>,
    mut reservations: ResMut<Reservations>,
    mut world_trees: ResMut<world::WorldTrees>,
) {
    if sim.paused {
        return;
    }

    let speed = sim.speed;
    sim.tick.tick(time.delta().mul_f32(speed));
    if !sim.tick.just_finished() {
        return;
    }

    for (entity, mut pawn, mut transform, mut task, mut inv) in &mut q {
        let next = match *task {
            Task::Idle => {
                pawn_tasks::handle_idle(entity, &pawn, &map, &mut reservations, &world_trees)
            }
            Task::GoToTree(at) => {
                pawn_tasks::handle_go_to_tree(&mut pawn, &mut transform, at)
            }
            Task::Chop { at, progress } => {
                pawn_tasks::handle_chop(
                    entity,
                    &mut map,
                    &mut inv,
                    at,
                    progress,
                    &mut reservations,
                    &mut world_trees,
                    &mut tile_entities,
                    &mut q_tiles,
                )
            }
            Task::GoToStockpile => pawn_tasks::handle_go_to_stockpile(&mut pawn, &mut transform),
            Task::DropOff => pawn_tasks::handle_drop_off(&mut inv, &mut stockpile),
        };

        *task = next;
    }
}
