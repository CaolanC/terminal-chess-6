mod game
{
    #[derive (Clone)]
    #[derive (Copy)]
    pub struct Piece
    {
        pub is_empty: bool,
        pub is_white: bool,
        pub is_black: bool,
        pub int_representation: u8,
    }

    impl Piece {

        fn bool_color(int_rep_color: char) -> bool {
            if int_rep_color == '1' {
                return true;
            }
            return false
        }

        pub fn new(str_fen: &String) -> Self {
            if str_fen.to_string() == "00" {
                Self {
                    is_empty: true,
                    is_white: false,
                    is_black: false,
                    int_representation: 0,
                }
            } else {
                let mut fen_chars = str_fen.chars();
                let s_int_rep = (fen_chars.next_back().expect("char unwrap")).to_string().parse::<u8>().unwrap();
                let color = fen_chars.next_back().expect("int-char unwrap");
                let mut s_empty = true;
                let mut s_is_white = false;
                if s_int_rep != 0 {
                    s_empty = false;
                    if Self::bool_color(color) {
                        s_is_white = false;
                    } else {
                        s_is_white = true;
                    }
                }
                Self {
                    is_empty: s_empty,
                    is_white: s_is_white,
                    is_black: !(s_is_white),
                    int_representation: s_int_rep,
                    
                }
            }
        }
    }

    pub struct Board
    {
        board: [[Piece; 8]; 8],
        fen_parse: String,
    }

    impl Board {

        pub fn debug_print(&self) {
            let mut board = "".to_string();
            for i in 0..8 {
                for j in 0..8 {
                    board.push_str(" ");
                    board.push_str(&self.board[i][j].int_representation.to_string().to_owned());
                }
                board.push_str("\n");
            }
            println!("{}", board.trim_end());
        }

        fn fill_fen_parsed(&mut self) {
            let parts = self.fen_parse.trim_end().split(' ');
            let collection = parts.collect::<Vec<&str>>();
            let segments: Vec<String> = collection.iter().map(|&s|(s.into())).collect();
            let mut i = 0;
            let mut j = 0;
            for segment in segments {
                if j == 8 {
                    j = 0;
                    i += 1;
                }
                self.board[i][j] = Piece::new(&segment);
                j += 1;
            }
        }

        pub fn fill_fen_custom_board(&mut self, fen: String) {
            let parts = fen.trim_end().split('\n');
            let collection = parts.collect::<Vec<&str>>();
            let strings: Vec<String> = collection.iter().map(|&s|(s.into())).collect();
            self.fen_parse = (&strings[0]).to_string();
            Self::fill_fen_parsed(self);

        }

        pub fn new_empty() -> Self {

            Self {
                board: [[Piece {
                    is_white: false,
                    is_black: false,
                    is_empty: true,
                    int_representation: 0, }
                    ; 8]; 8],
                fen_parse: "".to_string(),
                        }
        }

        pub fn default_fill(&mut self){
            for i in 0..8 {
                for j in 0..8
                {
                    self.board[i][j] = Piece {
                        is_empty: true,
                        is_white: false,
                        is_black: false,
                        int_representation: 0,
                    };
                };
            };
        }
    }
}

use game::Board;
use std::fs;
fn main() { 
    let mut x = Board::new_empty();
    let file = fs::read_to_string("../board_layouts/fen_custom_format/out.fen").expect("read file");
    Board::fill_fen_custom_board(&mut x, file);
    Board::debug_print(&mut x);
}
