use bevy::prelude::*;

use bevy::window::PrimaryWindow;

use crate::colony::Colony;
use crate::config::TILE_SIZE;
use crate::pawn::{Pawn, Task};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiTextTag {
    WoodValue,
    PawnAction,
    PawnPosition,
    PawnId,
}

#[derive(Resource, Default)]
pub struct SelectedPawn(pub Option<Entity>);

const PAWN_COLOR: Color = Color::srgb(0.85, 0.85, 0.95);
const PAWN_COLOR_SELECTED: Color = Color::srgb(1.0, 0.9, 0.4);

pub fn spawn_ui(commands: &mut Commands) {
    commands.insert_resource(SelectedPawn::default());
    spawn_colony_ui(commands);
    spawn_pawn_ui(commands);
}

pub fn spawn_colony_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new("Colony Inventory: "),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(8.0),
            ..default()
        },
    ));

    commands
        .spawn((
            Text::new("Wood: "),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(32.0),
                left: Val::Px(16.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::new("0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.8, 0.6)),
                UiTextTag::WoodValue,
            ));
        });
}

pub fn spawn_pawn_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new("Pawn Inspector"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            left: Val::Px(8.0),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("-------------"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(104.0),
            left: Val::Px(8.0),
            ..default()
        },
    ));

    commands
        .spawn((
            Text::new("ID:"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(128.0),
                left: Val::Px(16.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::new("?"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.85, 1.0)),
                UiTextTag::PawnId,
            ));
        });

    commands
        .spawn((
            Text::new("Position: "),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(152.0),
                left: Val::Px(16.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::new("(0,0)"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.85, 1.0)),
                UiTextTag::PawnPosition,
            ));
        });

    commands
        .spawn((
            Text::new("Action: "),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(176.0),
                left: Val::Px(16.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::new("Idle"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.85, 1.0)),
                UiTextTag::PawnAction,
            ));
        });
}

pub fn update_wood_ui(colony: Res<Colony>, mut q: Query<(&UiTextTag, &mut TextSpan)>) {
    if !colony.is_changed() {
        return;
    }

    for (tag, mut text) in &mut q {
        if *tag == UiTextTag::WoodValue {
            text.0 = colony.wood.to_string();
            break;
        }
    }
}

pub fn update_pawn_ui(
    selected: Res<SelectedPawn>,
    q_pawns: Query<(&Pawn, &Task)>,
    mut q_text: Query<(&UiTextTag, &mut TextSpan)>,
) {
    let (action_value, position_value, id_value) = match selected.0 {
        Some(entity) => match q_pawns.get(entity) {
            Ok((pawn, task)) => (
                format_task(task),
                format!("({},{})", pawn.x, pawn.y),
                pawn.id.to_string(),
            ),
            Err(_) => ("None".to_string(), "(?,?)".to_string(), "?".to_string()),
        },
        None => ("None".to_string(), "(?,?)".to_string(), "?".to_string()),
    };

    for (tag, mut text) in &mut q_text {
        match *tag {
            UiTextTag::PawnAction => text.0 = action_value.clone(),
            UiTextTag::PawnPosition => text.0 = position_value.clone(),
            UiTextTag::PawnId => text.0 = id_value.clone(),
            UiTextTag::WoodValue => {}
        }
    }
}

fn format_task(task: &Task) -> String {
    match *task {
        Task::Idle => "Idle".to_string(),
        Task::GoToTree(at) => format!("GoToTree ({},{})", at.x, at.y),
        Task::Chop { at, progress } => {
            format!("Chop ({},{}) {} %", at.x, at.y, progress.saturating_mul(10))
        }
        Task::GoToStockpile => "GoToStockpile".to_string(),
        Task::DropOff => "DropOff".to_string(),
    }
}

pub fn select_pawn_on_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    q_pawns: Query<(Entity, &Transform), With<Pawn>>,
    mut selected: ResMut<SelectedPawn>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = match windows.single() {
        Ok(window) => window,
        Err(_) => return,
    };

    let cursor_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };

    let (camera, camera_transform) = match cameras.single() {
        Ok(camera) => camera,
        Err(_) => return,
    };

    let world_pos = match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
        Ok(pos) => pos,
        Err(_) => return,
    };

    let mut best: Option<(f32, Entity)> = None;
    let radius = TILE_SIZE * 0.5;
    let radius_sq = radius * radius;

    for (entity, transform) in &q_pawns {
        let pawn_pos = transform.translation.truncate();
        let dist_sq = pawn_pos.distance_squared(world_pos);
        if dist_sq <= radius_sq {
            match best {
                None => best = Some((dist_sq, entity)),
                Some((best_dist, _)) if dist_sq < best_dist => best = Some((dist_sq, entity)),
                _ => {}
            }
        }
    }

    selected.0 = best.map(|(_, entity)| entity);
}

pub fn update_selected_pawn_visuals(
    selected: Res<SelectedPawn>,
    mut q_pawns: Query<(Entity, &mut Sprite), With<Pawn>>,
) {
    if !selected.is_changed() {
        return;
    }

    for (entity, mut sprite) in &mut q_pawns {
        sprite.color = if Some(entity) == selected.0 {
            PAWN_COLOR_SELECTED
        } else {
            PAWN_COLOR
        };
    }
}
