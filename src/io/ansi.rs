#![allow(unused)]

use std::fmt::Display;

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

    fn is_predefined(&self) -> bool {
        match *self {
            Colour::BLACK
            | Colour::RED
            | Colour::YELLOW
            | Colour::GREEN
            | Colour::BLUE
            | Colour::CYAN
            | Colour::MAGENTA
            | Colour::WHITE => true,

            _ => false,
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
}

pub struct Style(String);

impl Style {
    pub fn from(s: &str) -> Style {
        Style(s.to_string())
    }

    pub fn fg(mut self, colour: Colour) -> Style {
        let esc_code = if colour.is_predefined() {
            "["
        } else {
            "[38;5;"
        };

        self.0 = format!("\x1b{esc_code}{}m{}{RESET}", colour.0, self.0);
        self
    }

    pub fn bg(mut self, colour: Colour) -> Style {
        // bg colours are fg + 10, but if a custom 256 colour is used
        // then it's just the colour code
        let (esc_code, adjusted_colour) = if colour.is_predefined() {
            ("[", colour.0 + 10)
        } else {
            ("[48;5;", colour.0)
        };

        self.0 = format!("\x1b[{}m{}{RESET}", adjusted_colour, self.0);
        self
    }

    pub fn with(mut self, modifier: Modifier) -> Style {
        self.0 = format!("\x1b[{}m{}{RESET}", modifier.0, self.0);
        self
    }

    /* ---- Default colouring options ---- */
    pub fn info(self) -> Style {
        self.fg(Colour::CYAN)
    }

    pub fn success(self) -> Style {
        self.fg(Colour::GREEN)
    }

    pub fn warning(self) -> Style {
        self.fg(Colour::YELLOW)
    }

    pub fn error(self) -> Style {
        self.fg(Colour::RED)
    }

    /* ---- Default modifier options ---- */
    pub fn bold(self) -> Style {
        self.with(Modifier::BOLD)
    }

    pub fn italic(self) -> Style {
        self.with(Modifier::ITALIC)
    }

    pub fn underline(self) -> Style {
        self.with(Modifier::UNDERLINED)
    }

    // TODO add more modifiers, these are the most common so fine for now
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
