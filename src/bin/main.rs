use terminal_chess::io::ansi::{BgColour, FgColour, Modifier, Style};

fn start_game() {}

fn main() {
    let mut style = Style::from("This is a string with lots of styles!");

    println!(
        "{}",
        style
            .bg(BgColour::BLUE) // Some colours have shortcuts
            .fg(FgColour::RED) // Any 8-bit colour (see https://en.wikipedia.org/wiki/ANSI_escape_code)
            .bold() // Modifiers also have shortcuts
            .underline()
            .with(Modifier::ITALIC) // or can be added manually
    );
}
