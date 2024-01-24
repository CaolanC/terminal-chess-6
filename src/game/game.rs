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
                color: color_int, // 0 -> White; 1 -> Black; -1 -> Out of play.
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
        pub row: i8,
        pub collumn: i8,
        pub has_moved: bool,
    }

    impl Piece {

        fn bool_color(int_rep_color: char) -> bool {
            if int_rep_color == '1' {
                return true;
            }
            return false
        }

        pub fn new(str_fen: &str, row: i8, collumn: i8) -> Self {
            if str_fen.to_string() == "00" {
                Self {
                    is_empty: true,
                    is_white: false,
                    is_black: false,
                    int_representation: 0,
                    row: -1,
                    collumn: -1,
                    color: Color::new(-1),
                    has_moved: false,
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
                    has_moved: false,
                }
            }
        }
    }

    pub struct Move
    {
        pub from_piece: Piece,
        pub to_piece: Piece,
        pub coords: [usize; 4],
    }

    impl Move
    {
        pub fn new(from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Self {
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
        pub white_pieces: Vec<Piece>,
        pub black_pieces: Vec<Piece>,
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

        fn is_enemy(&self, piece: Piece) -> bool {
            if self.curr_color.color != piece.color.color && !piece.is_empty {
                return true
            }
            return false
        }

        fn is_diagonal_attacker(&self, piece: Piece) -> bool {
            if Self::is_enemy(&self, piece) && (piece.int_representation == 5 || piece.int_representation == 4) {
               return true;
            }

            return false;
        }

        fn check_scan_diagonals(&self, king_x: usize, king_y: usize) -> bool {

            for x in (king_x)..8_usize {
                for y in (king_y)..8_usize {
                    if Self::is_diagonal_attacker(&self, self.board[x][y]) {
                        return true;
                    } else if !(self.board[x][y].is_empty){
                        break;
                    }
                }
            }
            for x in (king_x)..8_usize {
                for y in (0..king_y).rev() {
                    if Self::is_diagonal_attacker(&self, self.board[x][y]) {
                        return true;
                    } else if !(self.board[x][y].is_empty){
                        break;
                    }
                }
            }
            for x in (0..king_x).rev() {
                for y in (king_y)..8 {
                    if Self::is_diagonal_attacker(&self, self.board[x][y]) {
                        return true;
                    } else if !(self.board[x][y].is_empty){
                        break;
                    }
                }
            }
            for x in (0..king_x).rev() {
                for y in (0..king_y).rev() {
                    if Self::is_diagonal_attacker(&self, self.board[x][y]) {
                        return true;
                    } else if !(self.board[x][y].is_empty){
                        break;
                    }
                }
            }

            return false;
        }

        fn is_line_attacker(&self, piece: Piece) -> bool {
            if Self::is_enemy(&self, piece) && (piece.int_representation == 5 || piece.int_representation == 2) {
               return true;
            }

            return false;
        }

        fn check_scan_lines(&self, king_x: usize, king_y: usize) -> bool {

            for x in (king_x + 1)..8 {    // Row Checking Right
                if Self::is_line_attacker(&self, self.board[x][king_y]) {
                    return true;
                } else if !(self.board[x][king_y].is_empty){
                    break;
                }
            }
            for x in (0..king_x).rev() { // Row Checking Left
                if Self::is_line_attacker(&self, self.board[x][king_y]) {
                    return true;
                } else if !(self.board[x][king_y].is_empty){
                    break;
                }
            }
            for y in (king_y + 1)..8 { // Collumn Checking Down
                if Self::is_line_attacker(&self, self.board[king_x][y]) {
                    return true;
                } else if !(self.board[king_x][y].is_empty) { 
                    break;
                }
            }
            for y in (0..(king_y - 1)).rev() { // Collumn Checking Up
                if Self::is_line_attacker(&self, self.board[king_x][y]) {
                    return true;
                } else if !(self.board[king_x][y].is_empty) {
                    break;
                }
            }

            return false
        }
        fn in_range(x: i8, y: i8) -> bool {
            if x < 8 && x >= 0 && y < 8 && y >= 0 {
                return true;
            };
            return false;
        }

        fn is_enemy_knight(&self, x: usize, y: usize) -> bool {
            if Self::is_enemy(&self, self.board[x][y]) && self.board[x][y].int_representation == 3 {
                return true;
            }
            return false;
        }

        fn check_scan_knight_squares(&self, kx: usize, ky: usize) -> bool { // kx, ky -> king_x, king_y
            if Self::in_range(kx as i8 + 2, ky as i8 + 1) && Self::is_enemy_knight(&self, kx + 2, ky + 1) {
                return true;
            }
            if Self::in_range(kx as i8 + 2, ky as i8 - 1) && Self::is_enemy_knight(&self, kx + 2, ky - 1) {
                return true;
            }
            if Self::in_range(kx as i8 + 1, ky as i8 + 2) && Self::is_enemy_knight(&self, kx + 1, ky + 2) {
                return true;
            }
            if Self::in_range(kx as i8 + 1, ky as i8 - 2) && Self::is_enemy_knight(&self, kx + 1, ky - 2) {
                return true;
            }
            // 
            if Self::in_range(kx as i8 - 1, ky as i8 + 2) && Self::is_enemy_knight(&self, kx - 1, ky + 2) {
                return true;
            }
            if Self::in_range(kx as i8 - 1, ky as i8 - 2) && Self::is_enemy_knight(&self, kx - 1, ky - 2) {
                return true;
            }
            if Self::in_range(kx as i8 - 2, ky as i8 + 1) && Self::is_enemy_knight(&self, kx - 2, ky + 1) {
                return true;
            }
            if Self::in_range(kx as i8 - 2, ky as i8 - 1) && Self::is_enemy_knight(&self, kx - 2, ky - 1) {
                return true;
            }
            return false;
        }

        fn is_enemy_pawn(&self, x: usize, y: usize) -> bool {
            if Self::is_enemy(&self, self.board[x][y]) && self.board[x][y].int_representation == 1 {
                return true;
            }
            return false;
        }

        fn check_pawn_checks(&self, kx: usize, ky: usize) -> bool {
            let mut dir: i8 = 1;
            if self.curr_color.color == 0 {
                dir = -1;
            }
            if Self::in_range(kx as i8 + dir, ky as i8 + 1) && Self::is_enemy_pawn(&self, kx + (dir as usize), ky + 1) {
                return true;
            }
            if Self::in_range(kx as i8 + dir, ky as i8 - 1) && Self::is_enemy_pawn(&self, kx + (dir as usize), ky - 1) {
                return true;
            }
            return false;

        }

        pub fn in_check(&self) -> bool {

            let king_x: usize;
            let king_y: usize;

            if self.t_white {
                king_x = self.white_king_position[0] as usize;
                king_y = self.white_king_position[1] as usize;
            } else {
                king_x = self.black_king_position[0] as usize;
                king_y = self.black_king_position[1] as usize;
            }

            println!("white - ({})\nkp: {}, {}", self.t_white, king_x, king_y);

            if Self::check_scan_diagonals(&self, king_x, king_y) {
                return true;
            }
            if Self::check_scan_lines(&self, king_x, king_y) {
                return true;
            }
            if Self::check_scan_knight_squares(&self, king_x, king_y) {
                return true;
            }
            if Self::check_pawn_checks(&self, king_x, king_y) {
                return true;
            }
            
            return false;
        }

        pub fn make_move(&mut self, mv: Move) {
            self.board[mv.coords[2] as usize][mv.coords[3] as usize] = self.board[mv.coords[0] as usize][mv.coords[1] as usize];
            self.board[mv.coords[0] as usize][mv.coords[1] as usize] = Piece::new(&"00".to_string(), mv.coords[0] as i8, mv.coords[1] as i8);
        }

        fn get_color_pieces(&mut self) {
            for row in 0..8 {
                for col in 0..8 {
                    if self.board[row][col].color.color == 0 {
                        self.white_pieces.push(self.board[row][col]);
                    } else if self.board[row][col].color.color == 1 {
                        self.black_pieces.push(self.board[row][col]);
                    }
                }
            }
        }

        fn check_possible_pawn_moves(&self, piece: Piece, color: i8) -> Vec<Move> {
            let row: usize = piece.row as usize;
            let col: usize = piece.collumn as usize;
            let mut moves = Vec::<Move>::new();
            let mut dir: i8 = -1;

            if color == 1 {
                dir = 1;
            }
            if Self::in_range((row as i8 + dir), col as i8) && (self.board[(row as i8+ dir) as usize][col].is_empty) {
                moves.push(Move::new(row, col, (row as i8 + dir) as usize, col));
                if Self::in_range((row as i8 + 2 * dir), col as i8) && (self.board[(row + (2 * dir) as usize)][col].is_empty) {
                    moves.push(Move::new(row, col, row + (2 * dir) as usize, col));
                }
            }

            if Self::in_range(row as i8 + dir, col as i8 + 1) && self.board[(row as i8 + dir) as usize][(col + 1) as usize].color.color != color &&
            !self.board[(row as i8 + dir) as usize][(col + 1) as usize].is_empty
            {
                moves.push(Move::new(row, col, (row as i8 + dir) as usize, col + 1));
            }

            if Self::in_range(row as i8 + dir, col as i8 - 1) && self.board[(row as i8 + dir) as usize][(col - 1)].color.color != color &&
            !self.board[(row as i8 + dir) as usize][(col - 1) as usize].is_empty
            {
                moves.push(Move::new(row, col, (row as i8 + dir) as usize, col - 1));
            }

            println!("{},{}: {}", row, col, moves.len());

            return moves;
        }

        fn check_possible_rook_moves(&self, piece: Piece, color: i8) {
            let row: usize = piece.row as usize;
            let col: usize = piece.collumn as usize;
            let mut moves = Vec::<Move>::new();

            for i in (row + 1)..8 {
                if self.board[i as usize][col].is_empty || self.board[i as usize][col].color.color != color {
                    moves.push(Move::new(row, col, i, col));
                }
            }

            for i in (col + 1)..8 {
            }

        }

        fn check_possible_moves(&self, piece: Piece, color: i8) {
            if piece.int_representation == 1 {
                Self::check_possible_pawn_moves(&self, piece, color);
            }

        }

        pub fn get_legal_moves(&mut self) {
            if self.curr_color.color == 1 {
                for piece in &self.black_pieces {
                    Self::check_possible_moves(&self, *piece, self.curr_color.color);
                }
            } else {
                for piece in &self.white_pieces {
                    Self::check_possible_moves(&self, *piece, self.curr_color.color);
                }
            }
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
                self.board[i][j] = Piece::new(&segment, i as i8, j as i8);
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
                    has_moved: false,
                }; 8]; 8],

                fen_parse: "".to_string(),
                white_king_position: [-1; 2],
                black_king_position: [-1; 2],
                curr_king_position: [-1, 2],
                enemy_king_position: [-1, 2],
                t_white: true,
                prev_move: Move::new(255,255,255,255),
                curr_color: Color::new(-1),
                white_pieces: Vec::new(),
                black_pieces: Vec::new(),

            }
        }

        pub fn default_new() -> Self {
            let mut def = Board::new();
            Board::fill_fen_custom_board(&mut def, fs::read_to_string("../../board_layouts/fen_custom_format/out.fen").expect("read file")); // to-do, update path .chess-config in home dir

            return def;
        }
    }
use std::fs;
fn main() { 
    let mut x = Board::new();
    let file = fs::read_to_string("../../board_layouts/fen_custom_format/out.fen").expect("read file"); // to-do, update path to .chess-config in home directory
    Board::fill_fen_custom_board(&mut x, file);
    //let mv: Move = Move::new(0,4,5,5);
    //Board::make_move(&mut x, mv);
    if Board::in_check(&x) {
        println!("In check");
    } else {
        println!("Not in check");
    }
    x.get_color_pieces();
    println!("{}", x.black_pieces.len());
    x.get_legal_moves();
    dbg!(&x);
    println!("FIN");
}