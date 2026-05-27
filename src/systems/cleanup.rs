use bevy::prelude::*;

use crate::components::{GameOverMessage, Human};

pub fn end_game(mut commands: Commands, entities: Query<Entity, With<Human>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }

    let container = Node {
        width: percent(100.0),
        height: percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let font_size: f32 = 64.0;

    // The fell through score/count
    let gameover_text = (
        GameOverMessage,
        Text::new("GAME OVER"),
        TextFont::from_font_size(font_size),
        TextColor(Color::srgb_u8(231, 95, 98)),
        TextLayout::new_with_justify(Justify::Center),
        Node { ..default() },
    );

    commands.spawn((container, children![gameover_text]));
}
