use bevy::prelude::*;
use crate::ui::styles;

// Creates a new Text_Bundle with text_style
pub fn text(text: &str, text_style: styles::TextStyle, asset_server: &AssetServer) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new(
                    text,
                    TextStyle {
                        font: asset_server.load(text_style.font),
                        font_size: text_style.size,
                        color: Color::hex(text_style.color_hex).unwrap(),
                    }
                )
            ],
            justify: text_style.justify,
            ..default()
        },
        ..default()
    }
}

// Creates a new ButtonBundle with button_style
pub fn button(button_style: styles::ButtonStyle) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: button_style.justify_content,
            align_items: button_style.align_items,
    
            width: button_style.width,
            height: button_style.height,
            ..Style::DEFAULT
        },
        background_color: Color::hex(button_style.color_hex).unwrap().into(),
        ..ButtonBundle::default()
    }
}
