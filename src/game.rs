mod game
{
    use std::fmt::Debug;

    #[derive (Clone)]
    #[derive (Copy)]
    pub struct Piece
    {
        pub is_empty: bool,
        pub is_white: bool,
        pub is_black: bool,
        pub int_representation: u8,
        pub row: i32,
        pub collumn: i32,
    }

    impl Piece {

        fn bool_color(int_rep_color: char) -> bool {
            if int_rep_color == '1' {
                return true;
            }
            return false
        }

        pub fn new(str_fen: &str, row: i32, collumn: i32) -> Self {
            if str_fen.to_string() == "00" {
                Self {
                    is_empty: true,
                    is_white: false,
                    is_black: false,
                    int_representation: 0,
                    row: -1,
                    collumn: -1,
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
                    row: row,
                    collumn: collumn,
                }
            }
        }
    }

    pub struct Board
    {
        pub board: [[Piece; 8]; 8],
        fen_parse: String,
        pub white_king_position: [i8; 2],
        pub black_king_position: [i8; 2],
        pub t_white: bool,
        pub curr_king_position: [i8; 2],
        pub enemy_king_position: [i8; 2],
        pub previous_move: [i8; 4],
        pub previous_piece: Piece,
    }

    impl Debug for Board {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut board = "".to_string();
            for i in 0..8 {
                for j in 0..8 {
                    board.push_str(" ");
                    board.push_str(&self.board[i][j].int_representation.to_string().to_owned());
              }
                board.push_str("\n");
            }
            write!(f, "Board:\n----------------\n{}\n----------------", board.trim_end());

            let mut board = "".to_string();
            for i in 0..8 {
                for j in 0..8 {
                    board.push_str(" ");
                    board.push_str(&self.board[i][j].is_white.to_string().to_owned());
                }
                board.push_str("\n");
            }
            write!(f, "Board:\n----------------\n{}\n----------------", board.trim_end())
        }

    }

    impl Board { // RULES | LOGIC | MOVEMENT

        fn check_scan_diagonals(king_x: i8, king_y: i8) {

            for horiz in 0..2 {
                let mut h_dir: u8 = 1;
                for verti in 0..2 {
                    let mut v_dir: u8 = 0;
                }
            }
        }

        pub fn in_check(&self) {

            let mut king_x: i8;
            let mut king_y: i8;

            if (self.t_white) {
                king_x = self.white_king_position[0];
                king_y = self.white_king_position[1];
            } else {
                king_x = self.black_king_position[0];
                king_y = self.black_king_position[1];
            }
        }

        pub fn make_move(&mut self, row_x: u8, col_x: u8, row_y: u8, col_y: u8) {
            self.board[row_y as usize][col_y as usize] = self.board[row_x as usize][row_x as usize];
            self.board[row_x as usize][col_x as usize] = Piece::new(&"00".to_string(), 0, 0); 
        }
    }

    impl Board { // UTIL AND CONSTRUCTION

        fn find_kings(&mut self) {
            for i in 0..8 {
                for j in 0..8 {
                    if self.board[i][j].int_representation == 6 {

                        if self.board[i][j].is_white {
                            let mut pos = [-1; 2];
                            pos[0] = i as i8;
                            pos[1] = j as i8;
                            self.white_king_position = pos;
                        } else {
                            let mut pos = [-1; 2];
                            pos[0] = i as i8;
                            pos[1] = j as i8;
                            self.black_king_position = pos;
                        }
                    }
                }
            }
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
                self.board[i][j] = Piece::new(&segment, i as i32, j as i32);
                j += 1;
            }
        }

        pub fn fill_fen_custom_board(&mut self, fen: String) {
            let parts = fen.trim_end().split('\n');
            let collection = parts.collect::<Vec<&str>>();
            let strings: Vec<String> = collection.iter().map(|&s|(s.into())).collect();
            self.fen_parse = (&strings[0]).to_string();
            Self::fill_fen_parsed(self);
            Self::find_kings(self);

        }

        pub fn new() -> Self {

            Self {
                board: [[Piece {
                    is_white: false,
                    is_black: false,
                    is_empty: true,
                    int_representation: 0,
                    row: -1,
                    collumn: -1,
                }
                    
                    ; 8]; 8],
                fen_parse: "".to_string(),
                white_king_position: [-1; 2],
                black_king_position: [-1; 2],
                curr_king_position: [-1, 2],
                enemy_king_position: [-1, 2],
                t_white: true,
                previous_move: [-1; 4],
                previous_piece: Piece::new("00", 0, 0),
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
                        row: -1,
                        collumn: -1,
                    };
                };
            };
        }
    }
}

use game::Board;
use std::fs;
fn main() { 
    let mut x = Board::new();
    let file = fs::read_to_string("../board_layouts/fen_custom_format/out.fen").expect("read file");
    Board::fill_fen_custom_board(&mut x, file);
    Board::make_move(&mut x, 0, 0, 5, 5);
    println!("x: {}, y: {}", x.black_king_position[0], x.black_king_position[1]);
    dbg!(&x);
}
