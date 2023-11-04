use std::fs;
use std::convert::TryInto;

#[allow(dead_code)]
pub struct Fen {
    unparsed_fen_vector: Vec<String>,
    board_fen: String,
    parsed_board_layout: [[u8; 8]; 8],
}   

impl Fen {

    fn fen_numeric_to_tcfen(c: char) -> String {
        let dig = c.to_digit(10).unwrap();
        let tcfen = "00 ".repeat(dig.try_into().unwrap());
        return tcfen;
    }

    fn fen_alphabetic_to_tcfen(c: char) -> String {
        
    }

    pub fn read_fen(&mut self, path: &str) {
    let contents = fs::read_to_string(path)
            .expect("File is read.");

        let parts = contents.split(" ");
        let collection = parts.collect::<Vec<&str>>();
        let strings: Vec<String> = collection.iter().map(|&s|(s.into())).collect();

        self.unparsed_fen_vector = strings;
        }

    pub fn parse_board_layout(&mut self)
    {
        let unparsed_layout: &String = &self.unparsed_fen_vector[0];
        let _ = unparsed_layout.trim();
        let mut tcfen: String = "".to_owned();
        for c in unparsed_layout.chars() {
            if c.is_numeric() {
                let zz_tcfen: &str = &Self::fen_numeric_to_tcfen(c);
                tcfen.push_str(zz_tcfen);
            } else if c.is_alphabetic() {
                println!("{}", c);
            }
        }
        println!("{}", tcfen);
    }

    pub fn new() -> Self {
        Self {
            board_fen: "".to_string(),
            parsed_board_layout: [[0; 8]; 8],
            unparsed_fen_vector: Vec::new(),
        }
    }

    pub fn print(&mut self) {
        println!("GTFO!");
    }

}

fn main() {
    let mut x = Fen::new();
    Fen::read_fen(&mut x, "../../board_layouts/fen_strings/default_fen.txt");
    Fen::parse_board_layout(&mut x);
}
