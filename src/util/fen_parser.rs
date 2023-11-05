use std::fs;
use std::convert::TryInto;
use std::char::from_digit;
use std::io::Write;

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
        
        let mut nv_fen: String = "".to_string();
        match c {
            'p'=>nv_fen.push_str("1 "),
            'r'=>nv_fen.push_str("2 "),
            'n'=>nv_fen.push_str("3 "),
            'b'=>nv_fen.push_str("4 "),
            'q'=>nv_fen.push_str("5 "),
            'k'=>nv_fen.push_str("6 "),
            'P'=>nv_fen.push_str("1 "),
            'R'=>nv_fen.push_str("2 "),
            'N'=>nv_fen.push_str("3 "),
            'B'=>nv_fen.push_str("4 "),
            'Q'=>nv_fen.push_str("5 "),
            'K'=>nv_fen.push_str("6 "),


             _ => unreachable!(),
        }
        return nv_fen
    }

    fn parse_white_or_black(c: &String) -> String {
        if c == "w" {
            return "\n1".to_string()
        };

        return "\n0".to_string();
    }

    fn parse_castling(castle: &String) -> String {
        let mut castling: String = "\n0000".to_string();

        for c in castle.chars() {
        match c {
            'K'=>castling.replace_range(1..2, "1"),
            'Q'=>castling.replace_range(2..3, "1"),
            'k'=>castling.replace_range(3..4, "1"),
            'q'=>castling.replace_range(4..5, "1"),
            '-'=>return "\n-".to_string(),

             _ => unreachable!(),
        }
    }

        return castling;
    }

    fn parse_en_passent(en_passent: &String) -> String {
        let mut en_passent_parsed: String = "\n".to_string();
        let mut en_p_chars = en_passent.chars();

        let char1 = en_p_chars.next_back().expect("char unwrap");
        if char1 == '-' {
            return "\n-".to_string();
        }

        let row = from_digit(8 - (char1.to_digit(10)).unwrap(), 10).unwrap().to_string();
        let char2 = en_p_chars.next_back().expect("char unwrap");
        let collumn = from_digit((char2 as u32) - 97, 10).expect("collumn unwrap").to_string();

        en_passent_parsed.push_str(&row);
        en_passent_parsed.push_str(" ");
        en_passent_parsed.push_str(&collumn);

        return en_passent_parsed;
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
                let mut vv_tcfen: String = "".to_owned();
                if c.is_uppercase() {
                    let v: &str = "0";
                    vv_tcfen.push_str(v);
                } else {
                    let v: &str = "1";
                    vv_tcfen.push_str(v);
                }
                let nv_fen: &str = &Self::fen_alphabetic_to_tcfen(c);
                vv_tcfen.push_str(nv_fen);
                tcfen.push_str(&vv_tcfen);
            }
        }
        let color: &String = &Self::parse_white_or_black(&self.unparsed_fen_vector[1]);
        tcfen.push_str(&color);

        let castling: &String = &Self::parse_castling(&self.unparsed_fen_vector[2]);
        tcfen.push_str(&castling);

        let en_passent: &String = &Self::parse_en_passent(&self.unparsed_fen_vector[3]);
        tcfen.push_str(&en_passent);
        println!("{}", tcfen);
        self.board_fen = tcfen;
    }

    pub fn new() -> Self {
        Self {
            board_fen: "".to_string(),
            parsed_board_layout: [[0; 8]; 8],
            unparsed_fen_vector: Vec::new(),
        }
    }

    pub fn write_fen_file(&mut self) {
        let file = fs::OpenOptions::new()
                                        .append(true)
                                        .open("../../board_layouts/fen_custom_format/out.fen");
    let _ = file.expect("file write").write_all(self.board_fen.as_bytes());

    }

}

fn main() {
    let mut x = Fen::new();
    Fen::read_fen(&mut x, "../../board_layouts/fen_strings/default.txt");
    Fen::parse_board_layout(&mut x);
}
