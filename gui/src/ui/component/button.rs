use iced::pure::{
    container, row,
    widget::{button, Container},
};
use iced::{Alignment, Color, Length, Vector};

use super::text::text;
use crate::ui::color;

pub fn alert<'a, T: 'a>(icon: Option<iced::Text>, t: &str) -> button::Button<'a, T> {
    button::Button::new(content(icon, t)).style(Style::Destructive)
}

pub fn primary<'a, T: 'a>(icon: Option<iced::Text>, t: &str) -> button::Button<'a, T> {
    button::Button::new(content(icon, t)).style(Style::Primary)
}

pub fn transparent<'a, T: 'a>(icon: Option<iced::Text>, t: &str) -> button::Button<'a, T> {
    button::Button::new(content(icon, t)).style(Style::Transparent)
}

pub fn transparent_border<'a, T: 'a>(icon: Option<iced::Text>, t: &str) -> button::Button<'a, T> {
    button::Button::new(content(icon, t)).style(Style::TransparentBorder)
}

fn content<'a, T: 'a>(icon: Option<iced::Text>, t: &str) -> Container<'a, T> {
    match icon {
        None => container(text(t)).width(Length::Fill).center_x().padding(5),
        Some(i) => container(
            row()
                .push(i)
                .push(text(t))
                .spacing(10)
                .width(iced::Length::Fill)
                .align_items(Alignment::Center),
        )
        .width(iced::Length::Fill)
        .center_x()
        .padding(5),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Style {
    Primary,
    Transparent,
    TransparentBorder,
    Border,
    Destructive,
}

impl button::StyleSheet for Style {
    fn active(&self) -> button::Style {
        match self {
            Style::Primary => button::Style {
                shadow_offset: Vector::default(),
                background: color::PRIMARY.into(),
                border_radius: 10.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                text_color: color::FOREGROUND,
            },
            Style::Destructive => button::Style {
                shadow_offset: Vector::default(),
                background: color::FOREGROUND.into(),
                border_radius: 10.0,
                border_width: 0.0,
                border_color: color::ALERT,
                text_color: color::ALERT,
            },
            Style::Transparent | Style::TransparentBorder => button::Style {
                shadow_offset: Vector::default(),
                background: Color::TRANSPARENT.into(),
                border_radius: 10.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                text_color: Color::BLACK,
            },
            Style::Border => button::Style {
                shadow_offset: Vector::default(),
                background: Color::TRANSPARENT.into(),
                border_radius: 10.0,
                border_width: 1.2,
                border_color: color::BACKGROUND,
                text_color: Color::BLACK,
            },
        }
    }

    fn hovered(&self) -> button::Style {
        match self {
            Style::Primary => button::Style {
                shadow_offset: Vector::default(),
                background: color::PRIMARY.into(),
                border_radius: 10.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                text_color: color::FOREGROUND,
            },
            Style::Destructive => button::Style {
                shadow_offset: Vector::default(),
                background: color::FOREGROUND.into(),
                border_radius: 10.0,
                border_width: 0.0,
                border_color: color::ALERT,
                text_color: color::ALERT,
            },
            Style::Transparent => button::Style {
                shadow_offset: Vector::default(),
                background: color::BACKGROUND.into(),
                border_radius: 10.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                text_color: Color::BLACK,
            },
            Style::TransparentBorder => button::Style {
                shadow_offset: Vector::default(),
                background: Color::TRANSPARENT.into(),
                border_radius: 10.0,
                border_width: 1.0,
                border_color: Color::BLACK,
                text_color: Color::BLACK,
            },
            Style::Border => button::Style {
                shadow_offset: Vector::default(),
                background: Color::TRANSPARENT.into(),
                border_radius: 10.0,
                border_width: 1.0,
                border_color: Color::BLACK,
                text_color: Color::BLACK,
            },
        }
    }
}
