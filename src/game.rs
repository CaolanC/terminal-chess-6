mod game
{
    use std::fmt::Debug;

    #[derive (Clone)]
    #[derive (Copy)]
    pub struct Color
    {
        pub color: i8,
    }

    impl Color
    {
        pub fn new(color_int: i8) -> Self {
            Self {
                color: color_int,
            }
        }

        pub fn invert(&mut self) {
            if self.color == -1 {
                println!("Color not inverted; color had value of -1");
            }
            if self.color == 0 {
                self.color = 1;
            } else {
                self.color = 0;
            }
        }
    }

    #[derive (Clone)]
    #[derive (Copy)]
    pub struct Piece
    {
        pub is_empty: bool,
        pub is_white: bool,
        pub is_black: bool,
        pub color: Color,
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
                    color: Color::new(-1),
                }
            } else {
                let mut fen_chars = str_fen.chars();
                let s_int_rep = (fen_chars.next_back().expect("char unwrap")).to_string().parse::<u8>().unwrap();
                let color = fen_chars.next_back().expect("int-char unwrap");
                let mut s_empty = true;
                let mut s_is_white = false;
                let mut s_int_color = -1;
                if s_int_rep != 0 {
                    s_empty = false;
                    if Self::bool_color(color) {
                        s_is_white = false;
                        s_int_color = 1;
                    } else {
                        s_is_white = true;
                        s_int_color = 0;
                    }
                }
                Self {
                    is_empty: s_empty,
                    is_white: s_is_white,
                    is_black: !(s_is_white),
                    int_representation: s_int_rep,
                    row: row,
                    collumn: collumn,
                    color: Color::new(s_int_color),
                }
            }
        }
    }

    pub struct Move
    {
        pub from_piece: Piece,
        pub to_piece: Piece,
        pub coords: [i8; 4],
    }

    impl Move
    {
        pub fn new(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> Self {
            Self {
                from_piece: Piece::new("00", 0, 0),
                to_piece: Piece::new("00", 0, 0),
                coords: [from_x, from_y, to_x, to_y],
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
        pub prev_move: Move,
        pub curr_color: Color,
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

        fn is_enemy(&self, piece: Piece) {

        }

        fn check_scan_diagonals(&self, king_x: i8, king_y: i8) {

            for x in king_x..8 {
                for y in king_y..8 {
                    println!("{}", self.board[x as usize][y as usize].int_representation)
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

            Board::check_scan_diagonals(&self, king_x, king_y);
        }

        pub fn make_move(&mut self, mv: Move) {
            self.board[mv.coords[2] as usize][mv.coords[3] as usize] = self.board[mv.coords[0] as usize][mv.coords[1] as usize];
            self.board[mv.coords[0] as usize][mv.coords[1] as usize] = Piece::new(&"00".to_string(), 0, 0);
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

        fn color_fen_parsed(&mut self, color: &str) {
            if color.to_string() == "1" {
            self.curr_color = Color::new(0);
            } else {
            self.curr_color = Color::new(1);
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
            Self::color_fen_parsed(self, &strings[1]);
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
                    color: Color::new(-1),
                }; 8]; 8],

                fen_parse: "".to_string(),
                white_king_position: [-1; 2],
                black_king_position: [-1; 2],
                curr_king_position: [-1, 2],
                enemy_king_position: [-1, 2],
                t_white: true,
                prev_move: Move::new(-1,-1,-1,-1),
                curr_color: Color::new(-1),

            }
        }
    }
}

use game::Board;
use game::Move;
use std::fs;
fn main() { 
    let mut x = Board::new();
    let file = fs::read_to_string("../board_layouts/fen_custom_format/out.fen").expect("read file");
    Board::fill_fen_custom_board(&mut x, file);
    let mv: Move = Move::new(0,4,5,5);
    //Board::make_move(&mut x, mv);
    Board::in_check(&x);
    dbg!(&x);
}
