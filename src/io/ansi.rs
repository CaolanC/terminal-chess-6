mod style;

pub use self::style::{Colours, Modifiers, Style};

#[macro_export]
macro_rules! styled {
    ($text:expr, $($mods:expr),+) => ({
        let mut style = Style::from($text);
        $(
            match $mods {
                "black" => style.colour(Colours::BLACK),
                "red" => style.colour(Colours::RED),
                "green" => style.colour(Colours::GREEN),
                "yellow" => style.colour(Colours::YELLOW),
                "blue" => style.colour(Colours::BLUE),
                "magenta" => style.colour(Colours::MAGENTA),
                "cyan" => style.colour(Colours::CYAN),
                "white" => style.colour(Colours::WHITE),

                "bg-black" => style.colour(Colours::BG_BLACK),
                "bg-red" => style.colour(Colours::BG_RED),
                "bg-green" => style.colour(Colours::BG_GREEN),
                "bg-yellow" => style.colour(Colours::BG_YELLOW),
                "bg-blue" => style.colour(Colours::BG_BLUE),
                "bg-magenta" => style.colour(Colours::BG_MAGENTA),
                "bg-cyan" => style.colour(Colours::BG_CYAN),
                "bg-white" => style.colour(Colours::BG_WHITE),

                "bold" => style.with(Modifiers::BOLD),
                "dim" => style.with(Modifiers::DIM),
                "italic" => style.with(Modifiers::ITALIC),
                "underlined" => style.with(Modifiers::UNDERLINED),
                "blink" => style.with(Modifiers::BLINK),
                "reverse" => style.with(Modifiers::REVERSE),
                "hidden" => style.with(Modifiers::HIDDEN),
                "strike" => style.with(Modifiers::STRIKE),
                _ => panic!("Invalid style option: {}", $mods)
            };
        )+
        style
    })
}

pub use styled;
