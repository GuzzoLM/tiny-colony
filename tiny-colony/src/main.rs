mod colony;
mod config;
mod pawn;
mod pawn_tasks;
mod sim;
mod world;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (sim::sim_controls, sim::tick_jobs))
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    // World (data + tiles rendered)
    let world = world::build_world();
    world::spawn_world_tiles(&mut commands, &world);
    commands.insert_resource(world);

    // Pawns
    pawn::spawn_pawns(&mut commands, &mut images);
}
