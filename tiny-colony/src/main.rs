mod colony;
mod config;
mod pawn;
mod pawn_tasks;
mod sim;
mod ui;
mod world;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                sim::sim_controls,
                sim::tick_jobs,
                ui::select_pawn_on_click,
                ui::update_selected_pawn_visuals,
                ui::update_wood_ui,
                ui::update_pawn_ui,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    ui::spawn_ui(&mut commands);

    let world = world::build_world();
    pawn::spawn_pawns(&mut commands, &mut images, &world);
    world::spawn_world_tiles(&mut commands, &world);
    commands.insert_resource(world);
}
