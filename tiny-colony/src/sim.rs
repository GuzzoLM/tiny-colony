use bevy::prelude::*;

use crate::pawn::Pawn;
use crate::world;

#[derive(Resource)]
pub struct Sim {
    pub paused: bool,
    pub speed: f32,
    pub tick: Timer,
    pub target: IVec2,
}

pub fn init(commands: &mut Commands) {
    commands.insert_resource(Sim {
        paused: false,
        speed: 1.0,
        tick: Timer::from_seconds(0.10, TimerMode::Repeating), // 10 Hz
        target: IVec2::new(12, 12),
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

pub fn move_pawn_0(time: Res<Time>, mut sim: ResMut<Sim>, mut q: Query<(&mut Pawn, &mut Transform)>) {
    if sim.paused {
        return;
    }

    let speed = sim.speed;
    sim.tick.tick(time.delta().mul_f32(speed));
    if !sim.tick.just_finished() {
        return;
    }

    for (mut pawn, mut transform) in &mut q {
        if pawn.id != 0 {
            continue;
        }

        let px = pawn.x;
        let py = pawn.y;

        let tx = sim.target.x;
        let ty = sim.target.y;

        if px == tx && py == ty {
            break;
        }

        if px < tx {
            pawn.x += 1;
        } else if px > tx {
            pawn.x -= 1;
        } else if py < ty {
            pawn.y += 1;
        } else if py > ty {
            pawn.y -= 1;
        }

        let pos = world::grid_to_world(pawn.x, pawn.y);
        transform.translation = pos + Vec3::new(0.0, 0.0, 1.0);

        break;
    }
}
