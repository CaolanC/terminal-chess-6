#![allow(unused)]

use std::fmt::Display;

pub const RESET: &'static str = "\x1b[0m";

pub struct Colours(i32);
impl Colours {
    pub const BLACK: &'static str = "\x1b[30m";
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const WHITE: &'static str = "\x1b[37m";

    pub const BG_BLACK: &'static str = "\x1b[40m";
    pub const BG_RED: &'static str = "\x1b[41m";
    pub const BG_GREEN: &'static str = "\x1b[42m";
    pub const BG_YELLOW: &'static str = "\x1b[43m";
    pub const BG_BLUE: &'static str = "\x1b[44m";
    pub const BG_MAGENTA: &'static str = "\x1b[45m";
    pub const BG_CYAN: &'static str = "\x1b[46m";
    pub const BG_WHITE: &'static str = "\x1b[47m";
}

pub struct Modifiers(i32);
impl Modifiers {
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
    pub const ITALIC: &'static str = "\x1b[3m";
    pub const UNDERLINED: &'static str = "\x1b[4m";
    pub const BLINK: &'static str = "\x1b[5m";
    pub const REVERSE: &'static str = "\x1b[7m";
    pub const HIDDEN: &'static str = "\x1b[8m";
    pub const STRIKE: &'static str = "\x1b[9m";
}

pub struct Style(Vec<&'static str>);

impl Style {
    fn has_reset(&self) -> bool {
        match self.0.last() {
            Some(s) => *s == RESET,
            None => false,
        }
    }

    fn add_reset(&mut self) {
        if !self.has_reset() {
            self.0.push(RESET);
        }
    }

    pub fn from(s: &'static str) -> Self {
        Self(vec![s])
    }

    pub fn colour(&mut self, colour: &'static str) -> &mut Self {
        self.0.insert(0, colour);
        self.add_reset();
        self
    }

    pub fn with(&mut self, modifier: &'static str) -> &mut Self {
        self.0.insert(0, modifier);
        self.add_reset();
        self
    }

    /* ---- Default colouring options ---- */
    pub fn info(&mut self) -> &mut Self {
        self.colour(Colours::CYAN)
    }

    pub fn success(&mut self) -> &mut Self {
        self.colour(Colours::GREEN)
    }

    pub fn warning(&mut self) -> &mut Self {
        self.colour(Colours::YELLOW)
    }

    pub fn error(&mut self) -> &mut Self {
        self.colour(Colours::RED)
    }

    /* ---- Default modifier options ---- */
    pub fn bold(&mut self) -> &mut Self {
        self.with(Modifiers::BOLD)
    }

    pub fn italic(&mut self) -> &mut Self {
        self.with(Modifiers::ITALIC)
    }

    pub fn underlined(&mut self) -> &mut Self {
        self.with(Modifiers::UNDERLINED)
    }

    // TODO add more modifiers, these are the most common so fine for now
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(""))
    }
}
