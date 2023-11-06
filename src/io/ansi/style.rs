#![allow(unused)]

use std::fmt::{Debug, Display};

const RESET: &str = "\x1b[0m";

#[derive(PartialEq, Eq)]
pub struct Colour(pub u8);

impl Colour {
    pub const BLACK: Colour = Colour(30);
    pub const RED: Colour = Colour(31);
    pub const GREEN: Colour = Colour(32);
    pub const YELLOW: Colour = Colour(33);
    pub const BLUE: Colour = Colour(34);
    pub const MAGENTA: Colour = Colour(35);
    pub const CYAN: Colour = Colour(36);
    pub const WHITE: Colour = Colour(37);
    pub const RESET: Colour = Colour(0);

    fn is_predefined(&self) -> bool {
        match *self {
            Colour::BLACK
            | Colour::RED
            | Colour::YELLOW
            | Colour::GREEN
            | Colour::BLUE
            | Colour::CYAN
            | Colour::MAGENTA
            | Colour::WHITE
            | Colour::RESET => true,

            _ => false,
        }
    }

    fn name(&self) -> String {
        match *self {
            Colour::BLACK => "Black".to_string(),
            Colour::RED => "Red".to_string(),
            Colour::YELLOW => "Yellow".to_string(),
            Colour::GREEN => "Green".to_string(),
            Colour::BLUE => "Blue".to_string(),
            Colour::CYAN => "Cyan".to_string(),
            Colour::MAGENTA => "Magenta".to_string(),
            Colour::WHITE => "White".to_string(),
            Colour::RESET => "Reset".to_string(),
            _ => self.0.to_string(),
        }
    }

    pub fn escaped_fg(&self) -> String {
        if self.is_predefined() {
            return format!("\x1b[{}m", self.0);
        } else {
            return format!("\x1b[38;5;{}m", self.0);
        }
    }

    pub fn escaped_bg(&self) -> String {
        if self.is_predefined() {
            return format!("\x1b[{}m", self.0 + 10);
        } else {
            return format!("\x1b[48;5;{}m", self.0);
        }
    }
}

impl Debug for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_predefined() {
            write!(f, "{}", self.name())
        } else {
            write!(f, "Colour({})", self.0)
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Modifier(u8);

impl Modifier {
    pub const BOLD: Modifier = Modifier(1);
    pub const DIM: Modifier = Modifier(2);
    pub const ITALIC: Modifier = Modifier(3);
    pub const UNDERLINED: Modifier = Modifier(4);
    pub const BLINK: Modifier = Modifier(5);
    pub const REVERSE: Modifier = Modifier(7);
    pub const HIDDEN: Modifier = Modifier(8);
    pub const STRIKE: Modifier = Modifier(9);
    pub const RESET: Modifier = Modifier(0);

    fn name(&self) -> String {
        match *self {
            Modifier::BOLD => "Bold".to_string(),
            Modifier::DIM => "Dim".to_string(),
            Modifier::ITALIC => "Italic".to_string(),
            Modifier::UNDERLINED => "Underlined".to_string(),
            Modifier::BLINK => "Blink".to_string(),
            Modifier::REVERSE => "Reverse".to_string(),
            Modifier::HIDDEN => "Hidden".to_string(),
            Modifier::STRIKE => "Strike".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn escaped(&self) -> String {
        format!("\x1b[{}m", self.0)
    }
}

impl Debug for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug)]
enum Representation {
    FgColour(Colour),
    BgColour(Colour),
    Modifier(Modifier),
    Text(String),
}

#[derive(Debug)]
pub struct Style(Vec<Representation>);

impl Style {
    fn has_reset(&self) -> bool {
        match self.0.last() {
            Some(Representation::FgColour(Colour::RESET)) => true,
            _ => false,
        }
    }

    fn add_reset(&mut self) {
        if !self.has_reset() {
            self.0.push(Representation::FgColour(Colour::RESET));
        }
    }

    pub fn from(s: &str) -> Self {
        Self(vec![Representation::Text(s.to_string())])
    }

    pub fn fg(&mut self, colour: Colour) -> &mut Self {
        self.0.insert(0, Representation::FgColour(colour));
        self.add_reset();
        self
    }

    pub fn bg(&mut self, colour: Colour) -> &mut Self {
        self.0.insert(0, Representation::BgColour(colour));
        self.add_reset();
        self
    }

    pub fn with(&mut self, modifier: Modifier) -> &mut Self {
        self.0.insert(0, Representation::Modifier(modifier));
        self.add_reset();
        self
    }

    /* ---- Default colouring options ---- */
    pub fn info(&mut self) -> &mut Self {
        self.fg(Colour::CYAN)
    }

    pub fn success(&mut self) -> &mut Self {
        self.fg(Colour::GREEN)
    }

    pub fn warning(&mut self) -> &mut Self {
        self.fg(Colour::YELLOW)
    }

    pub fn error(&mut self) -> &mut Self {
        self.fg(Colour::RED)
    }

    /* ---- Default modifier options ---- */
    pub fn bold(&mut self) -> &mut Self {
        self.with(Modifier::BOLD)
    }

    pub fn italic(&mut self) -> &mut Self {
        self.with(Modifier::ITALIC)
    }

    pub fn underline(&mut self) -> &mut Self {
        self.with(Modifier::UNDERLINED)
    }

    // TODO add more modifiers, these are the most common so fine for now
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for repr in &self.0 {
            match repr {
                Representation::FgColour(colour) => output.push_str(&colour.escaped_fg()),
                Representation::BgColour(colour) => output.push_str(&colour.escaped_bg()),
                Representation::Modifier(modifier) => output.push_str(&modifier.escaped()),
                Representation::Text(text) => output.push_str(text),
            };
        }
        write!(f, "{}", output)
    }
}
