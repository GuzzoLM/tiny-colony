use bevy::prelude::*;

use crate::colony::Colony;
use crate::pawn::{Pawn, Task};

#[derive(Component)]
pub struct WoodValueText;

#[derive(Component)]
pub struct PawnAction;

#[derive(Component)]
pub struct PawnPosition;

#[derive(Component)]
pub struct PawnId;

pub fn spawn_ui(commands: &mut Commands) {
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

    commands.spawn((
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
            WoodValueText,
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

    commands.spawn((
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
            PawnId,
        ));
    });

    commands.spawn((
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
            PawnPosition,
        ));
    });

    commands.spawn((
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
            PawnAction,
        ));
    });
}

pub fn update_wood_ui(
    colony: Res<Colony>,
    mut q: Query<&mut TextSpan, With<WoodValueText>>,
) {
    if !colony.is_changed() {
        return;
    }

    if let Ok(mut text) = q.single_mut() {
        text.0 = colony.wood.to_string();
    }
}

pub fn update_pawn_ui(
    q_pawns: Query<(&Pawn, &Task)>,
    mut q_text: ParamSet<(
        Query<&mut TextSpan, With<PawnAction>>,
        Query<&mut TextSpan, With<PawnPosition>>,
        Query<&mut TextSpan, With<PawnId>>,
    )>,
) {
    let mut action_value = None;
    let mut position_value = None;
    let mut id_value = None;

    for (pawn, task) in &q_pawns {
        if pawn.id != 0 {
            continue;
        }

        action_value = Some(format_task(task));
        position_value = Some(format!("({},{})", pawn.x, pawn.y));
        id_value = Some(pawn.id.to_string());
        break;
    }

    let (
        Some(action_value),
        Some(position_value),
        Some(id_value)) = (action_value, position_value, id_value) else {
        return;
    };

    if let Ok(mut action_text) = q_text.p0().single_mut() {
        action_text.0 = action_value;
    }

    if let Ok(mut position_text) = q_text.p1().single_mut() {
        position_text.0 = position_value;
    }

    if let Ok(mut id_text) = q_text.p2().single_mut() {
        id_text.0 = id_value;
    }
}

fn format_task(task: &Task) -> String {
    match *task {
        Task::Idle => "Idle".to_string(),
        Task::GoToTree(at) => format!("GoToTree ({},{})", at.x, at.y),
        Task::Chop { at, progress } => format!("Chop ({},{}) {} %", at.x, at.y, progress.saturating_mul(10)),
        Task::GoToStockpile => "GoToStockpile".to_string(),
        Task::DropOff => "DropOff".to_string(),
    }
}
