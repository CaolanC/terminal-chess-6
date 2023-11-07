#![allow(unused)]

use std::fmt::{Debug, Display};

pub const RESET: &'static str = "\x1b[0m";

#[derive(PartialEq, Eq)]
pub struct FgColour(&'static str);

impl FgColour {
    pub const BLACK: &FgColour = &FgColour("30");
    pub const RED: &FgColour = &FgColour("31");
    pub const GREEN: &FgColour = &FgColour("32");
    pub const YELLOW: &FgColour = &FgColour("33");
    pub const BLUE: &FgColour = &FgColour("34");
    pub const MAGENTA: &FgColour = &FgColour("35");
    pub const CYAN: &FgColour = &FgColour("36");
    pub const WHITE: &FgColour = &FgColour("37");

    fn escaped(&self) -> String {
        format!("\x1b[{}m", self.0)
    }
}

impl Display for FgColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct BgColour(&'static str);

impl BgColour {
    pub const BLACK: &BgColour = &BgColour("40");
    pub const RED: &BgColour = &BgColour("41");
    pub const GREEN: &BgColour = &BgColour("42");
    pub const YELLOW: &BgColour = &BgColour("43");
    pub const BLUE: &BgColour = &BgColour("44");
    pub const MAGENTA: &BgColour = &BgColour("45");
    pub const CYAN: &BgColour = &BgColour("46");
    pub const WHITE: &BgColour = &BgColour("47");

    fn escaped(&self) -> String {
        format!("\x1b[{}m", self.0)
    }
}

impl Display for BgColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct Modifier(&'static str);

impl Modifier {
    pub const BOLD: &Modifier = &Modifier("1");
    pub const DIM: &Modifier = &Modifier("2");
    pub const ITALIC: &Modifier = &Modifier("3");
    pub const UNDERLINED: &Modifier = &Modifier("4");
    pub const BLINK: &Modifier = &Modifier("5");
    pub const REVERSE: &Modifier = &Modifier("7");
    pub const HIDDEN: &Modifier = &Modifier("8");
    pub const STRIKE: &Modifier = &Modifier("9");

    fn escaped(&self) -> String {
        format!("\x1b[{}m", self.0)
    }
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Text(&'static str);

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

    pub fn fg(&mut self, colour: &'static FgColour) -> &mut Self {
        self.0.insert(0, colour.0);
        self.add_reset();
        self
    }

    pub fn bg(&mut self, colour: &'static BgColour) -> &mut Self {
        self.0.insert(0, colour.0);
        self.add_reset();
        self
    }

    pub fn with(&mut self, modifier: &'static Modifier) -> &mut Self {
        self.0.insert(0, modifier.0);
        self.add_reset();
        self
    }

    /* ---- Default colouring options ---- */
    pub fn info(&mut self) -> &mut Self {
        self.fg(&FgColour::CYAN)
    }

    pub fn success(&mut self) -> &mut Self {
        self.fg(&FgColour::GREEN)
    }

    pub fn warning(&mut self) -> &mut Self {
        self.fg(&FgColour::YELLOW)
    }

    pub fn error(&mut self) -> &mut Self {
        self.fg(&FgColour::RED)
    }

    /* ---- Default modifier options ---- */
    pub fn bold(&mut self) -> &mut Self {
        self.with(&Modifier::BOLD)
    }

    pub fn italic(&mut self) -> &mut Self {
        self.with(&Modifier::ITALIC)
    }

    pub fn underline(&mut self) -> &mut Self {
        self.with(&Modifier::UNDERLINED)
    }

    // TODO add more modifiers, these are the most common so fine for now
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for style in &self.0 {
            s.push_str(style);
        }
        write!(f, "{}", s)
    }
}
