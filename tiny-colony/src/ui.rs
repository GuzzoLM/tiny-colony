use bevy::prelude::*;

use crate::colony::Colony;
use crate::pawn::{Pawn, Task};

#[derive(Component)]
pub struct WoodValueText;

#[derive(Component)]
pub struct Pawn0Text;

pub fn spawn_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new("Wood: "),
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

    commands.spawn((
        Text::new("Pawn0: "),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(32.0),
            left: Val::Px(8.0),
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
            Pawn0Text,
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

pub fn update_pawn0_ui(
    q_pawn0: Query<(&Pawn, &Task)>,
    mut q_text: Query<&mut TextSpan, With<Pawn0Text>>,
) {
    let Ok(mut text) = q_text.single_mut() else { return; };

    for (pawn, task) in &q_pawn0 {
        if pawn.id != 0 {
            continue;
        }

        text.0 = format_task(task);
        break;
    }
}

fn format_task(task: &Task) -> String {
    match *task {
        Task::Idle => "Idle".to_string(),
        Task::GoToTree(at) => format!("GoToTree ({},{})", at.x, at.y),
        Task::Chop { at, progress } => format!("Chop ({},{}) {}/10", at.x, at.y, progress),
        Task::GoToStockpile => "GoToStockpile".to_string(),
        Task::DropOff => "DropOff".to_string(),
    }
}
