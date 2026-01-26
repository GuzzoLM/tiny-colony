use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::config::*;
use crate::world;

#[derive(Component)]
pub struct Pawn {
    pub id: u32,
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum Task {
    Idle,
    GoToTree(IVec2),
    Chop { at: IVec2, progress: u8 },
    GoToStockpile,
    DropOff,
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Inventory {
    pub wood: u32,
}

pub fn spawn_pawns(commands: &mut Commands, images: &mut ResMut<Assets<Image>>) {
    let circle_image = images.add(make_circle_image(PAWN_RADIUS_PX));

    let spawn_offsets: [(i32, i32); PAWN_COUNT] = [
        (1, 0), (-1, 0), (0, 1), (0, -1),
        (1, 1), (-1, 1), (1, -1), (-1, -1),
        (2, 0), (-2, 0),
    ];

    for (i, (dx, dy)) in spawn_offsets.into_iter().enumerate() {
        let x = STOCKPILE_X + dx;
        let y = STOCKPILE_Y + dy;

        let pos = world::grid_to_world(x, y);
        let transform = Transform::from_translation(pos + Vec3::new(0.0, 0.0, 1.0));

        commands.spawn((
            Pawn { id: i as u32, x, y },
            Sprite {
                image: circle_image.clone(),
                color: Color::srgb(0.85, 0.85, 0.95),
                custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                ..default()
            },
            transform,
        ))
        .insert(Task::Idle)
        .insert(Inventory::default());
    }

    crate::sim::init(commands);
}

fn make_circle_image(radius: u32) -> Image {
    let size = radius * 2 + 2;
    let w = size as usize;
    let h = size as usize;

    let mut data = vec![0u8; w * h * 4];

    let cx = (size as f32) / 2.0;
    let cy = (size as f32) / 2.0;
    let r = radius as f32;

    for y in 0..h {
        for x in 0..w {
            let fx = x as f32 + 0.5;
            let fy = y as f32 + 0.5;

            let dx = fx - cx;
            let dy = fy - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            let inside = dist <= r;

            let idx = (y * w + x) * 4;
            data[idx + 0] = 255;
            data[idx + 1] = 255;
            data[idx + 2] = 255;
            data[idx + 3] = if inside { 255 } else { 0 };
        }
    }

    Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}
