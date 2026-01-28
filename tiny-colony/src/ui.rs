use bevy::prelude::*;

use crate::colony::Colony;
use crate::pawn::{Pawn, Task};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiTextTag {
    WoodValue,
    PawnAction,
    PawnPosition,
    PawnId,
}

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
    q_pawns: Query<(&Pawn, &Task), Or<(Changed<Pawn>, Changed<Task>)>>,
    mut q_text: Query<(&UiTextTag, &mut TextSpan)>,
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

    let (Some(action_value), Some(position_value), Some(id_value)) =
        (action_value, position_value, id_value)
    else {
        return;
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
