use bevy::prelude::*;

pub const FONT_PATH: &'static str = "fonts/Roboto-Medium.ttf";

pub const BACKGROUND_HEX: &'static str = "1a0923";
pub const FOREGROUND_HEX: &'static str = "33293e";
pub const TEXT_HEX: &'static str = "ffffff";
pub const TEXT_BOLD_HEX: &'static str = "cfeb73";

pub const BUTTON_DEFAULT_HEX: &'static str = "33293e";
pub const BUTTON_HOVER_HEX: &'static str = "2c2336";
pub const BUTTON_PRESSED_HEX: &'static str = "282030";

pub struct TextStyle {
    pub size: f32,
    pub color_hex: &'static str,
    pub font: &'static str,
    pub justify: JustifyText,
}

pub struct ButtonStyle {
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,

    pub width: Val,
    pub height: Val,

    pub color_hex: &'static str,

    pub text_style: Option<TextStyle>,
}


pub const TITLE_TEXT_STYLE: TextStyle = TextStyle {
    size: 96.0,
    color_hex: TEXT_BOLD_HEX,
    font: FONT_PATH,
    justify: JustifyText::Center,
};

pub const BODY_TEXT_STYLE: TextStyle = TextStyle {
    size: 48.0,
    color_hex: TEXT_HEX,
    font: FONT_PATH,
    justify: JustifyText::Center,
};

// Ui Button style
pub const BUTTON_STYLE: ButtonStyle = ButtonStyle {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,

    width: Val::Percent(30.0),
    height: Val::Percent(8.0),

    color_hex: BUTTON_DEFAULT_HEX,

    text_style: Some(BODY_TEXT_STYLE),
};

// Reward Button style
pub const REWARD_BUTTON_STYLE: ButtonStyle = ButtonStyle {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,

    width: Val::Percent(40.0),
    height: Val::Percent(60.0),

    color_hex: BUTTON_DEFAULT_HEX,

    text_style: Some(BODY_TEXT_STYLE),
};


