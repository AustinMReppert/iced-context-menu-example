use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{Background, Theme};

pub struct ContextMenuStyleSheet;

impl ContextMenuStyleSheet {
    pub fn new() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(ContextMenuStyleSheet {}))
    }
}

impl button::StyleSheet for ContextMenuStyleSheet {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: Some(Background::Color(
                style.extended_palette().secondary.base.color,
            )),
            border_radius: Default::default(),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: style.palette().text,
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: Some(Background::Color(
                style.extended_palette().primary.base.color,
            )),
            border_radius: Default::default(),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: style.palette().text.inverse(),
        }
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: Some(Background::Color(
                style.extended_palette().primary.strong.color,
            )),
            border_radius: Default::default(),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: style.palette().text.inverse(),
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: Some(Background::Color(
                style.extended_palette().secondary.weak.color,
            )),
            border_radius: Default::default(),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: style.palette().text,
        }
    }
}
