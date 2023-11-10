// relevant imports for styled! macro must be imported
use chess::io::ansi::{Colours, Modifiers, Style};

// All macros are/will be available in the root namespace
use chess::styled;

fn start_game() {}

fn main() {
    // see src/io/ansi.rs for all available styles
    let output = styled!("Hello, world!", "blue", "bg-black", "bold", "underlined");
    println!("{}", output);
}
