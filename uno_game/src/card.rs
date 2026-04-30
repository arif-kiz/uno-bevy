use std::fmt::Display;

use card_shuffling::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnoColor {
    Yellow,
    Green,
    Blue,
    Red,
    #[default]
    Wild,
}

impl Display for UnoColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let write = match self {
            Self::Yellow => "yellow",
            Self::Green => "green",
            Self::Blue => "blue",
            Self::Red => "red",
            Self::Wild => "wild",
        };
        f.write_fmt(format_args!("{}", write))
    }
}

impl Color for UnoColor {
    fn from_string(s: &str) -> Self {
        match s {
            "yellow" => UnoColor::Yellow,
            "green" => UnoColor::Green,
            "blue" => UnoColor::Blue,
            "red" => UnoColor::Red,
            "wild" => UnoColor::Wild,
            _ => panic!("Invalid color"),
        }
    }

    fn is_wild(&self) -> bool {
        matches!(self, UnoColor::Wild)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnoAction {
    Number(u8),
    Skip,
    Reverse,
    DrawTwo,
    #[default]
    ColorChange,
    DrawFour,
}

impl Action for UnoAction {
    fn from_string(s: &str) -> Self {
        match s {
            "skip" => UnoAction::Skip,
            "reverse" => UnoAction::Reverse,
            "draw_two" => UnoAction::DrawTwo,
            "color_change" => UnoAction::ColorChange,
            "draw_four" => UnoAction::DrawFour,
            _ => UnoAction::Number(s.parse().unwrap()),
        }
    }

    fn power(self) -> i32 {
        match self {
            UnoAction::Number(_) => 1,
            UnoAction::Skip => 2,
            UnoAction::Reverse => 2,
            UnoAction::DrawTwo => 4,
            UnoAction::ColorChange => 6,
            UnoAction::DrawFour => 8,
        }
    }
}

impl Display for UnoAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let write = match self {
            Self::Number(n) => &n.to_string(),
            Self::Skip => "skip",
            Self::Reverse => "reverse",
            Self::DrawTwo => "draw_two",
            Self::ColorChange => "color_change",
            Self::DrawFour => "draw_four",
        };
        f.write_fmt(format_args!("{}", write))
    }
}