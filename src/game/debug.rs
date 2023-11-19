mod game;
use game::Board;
use game::Piece;
use game::Move;
use game::Color;
impl Board // DEBUG
{

    fn debug_print(&self, container: Vec<[i8; 2]>) {
        let board = [[-1; 8]; 8];
        let mut str_board: String = "".to_string();
        for x in 0..8 {
            for y in 0..8 {
                if container.contains(&[x, y]) {
                    str_board.push_str("00");
                } else {
                    str_board.push_str("11");
                }
            }
            str_board.push_str("\n");
        }
        println!("{}", str_board);
    }

        fn d_is_enemy(&self, piece: Piece) -> bool {
            if self.curr_color.color != piece.color.color && !piece.is_empty{
                return true;
            }
            return false;
        }

        fn d_is_diagonal_attacker(&self, piece: Piece) -> bool {
            if Self::d_is_enemy(&self, piece) && (piece.int_representation == 5 || piece.int_representation == 4) {
               return true;
            }

            return false;
        }

        fn d_check_scan_diagonals(&self, king_x: i8, king_y: i8, Vec<[i8; 2]>) -> bool {

            for x in king_x..8 {
                for y in king_y..8 {
                    if Self::d_is_diagonal_attacker(&self, self.board[king_x as usize][king_y as usize]) {
                        return true;
                    } else if !(Self::d_is_enemy(&self, self.board[king_x as usize][king_y as usize])){
                        break;
                    }
                }
            }
            for x in king_x..8 {
                for y in (0..king_y).rev() {
                    if Self::d_is_diagonal_attacker(&self, self.board[king_x as usize][king_y as usize]) {
                        return true;
                    } else if !(Self::d_is_enemy(&self, self.board[king_x as usize][king_y as usize])){
                        break;
                    }
                }
            }
            for x in (0..king_x).rev() {
                for y in king_y..8 {
                    if Self::d_is_diagonal_attacker(&self, self.board[king_x as usize][king_y as usize]) {
                        return true;
                    } else if !(Self::d_is_enemy(&self, self.board[king_x as usize][king_y as usize])){
                        break;
                    }
                }
            }
            for x in (0..king_x).rev() {
                for y in (0..king_y).rev() {
                    if Self::d_is_diagonal_attacker(&self, self.board[king_x as usize][king_y as usize]) {
                        return true;
                    } else if !(Self::d_is_enemy(&self, self.board[king_x as usize][king_y as usize])){
                        break;
                    }
                }
            }

            return false;
        }

        fn d_is_line_attacker(&self, piece: Piece) -> bool {
            if Self::d_is_enemy(&self, piece) && (piece.int_representation == 5 || piece.int_representation == 2) {
               return true;
            }

            return false;
        }

        fn d_check_scan_lines(&self, king_x: i8, king_y: i8) -> bool {

            for x in king_x..8 {    // Row Checking Right
                if Self::d_is_line_attacker(&self, self.board[king_x as usize][x as usize]) {
                    return true;
                } else if !(Self::d_is_enemy(&self, self.board[king_x as usize][x as usize])){
                    break;
                }
            }
            for x in (king_x..8).rev() { // Row Checking Left
                if Self::d_is_line_attacker(&self, self.board[king_x as usize][x as usize]) {
                    return true;
                } else if !(Self::d_is_enemy(&self, self.board[king_x as usize][x as usize])){
                    break;
                }
            }
            for y in king_y..8 { // Collumn Checking Up
                if Self::d_is_line_attacker(&self, self.board[king_y as usize][y as usize]) {
                    return true;
                } else if !(Self::d_is_enemy(&self, self.board[king_y as usize][y as usize])){
                    break;
                }
            }
            for y in (king_y..8).rev() { // Collumn Checking Down
                if Self::d_is_line_attacker(&self, self.board[king_y as usize][y as usize]) {
                    return true;
                } else if !(Self::d_is_enemy(&self, self.board[king_y as usize][y as usize])){
                    break;
                }
            }

            return false
        }
        fn d_in_range(x: i8, y: i8) -> bool {
            if x < 8 && x > 0 && y < 8 && y > 0 {
                return true;
            };
            return false;
        }

        fn d_is_enemy_knight(&self, x: i8, y: i8) -> bool {
            if Self::d_is_enemy(&self, self.board[x as usize][y as usize]) && self.board[x as usize][y as usize].int_representation == 3 {
                return true;
            }
            return false;
        }

        fn d_check_scan_knight_squares(&self, kx: i8, ky: i8) -> bool { // kx, ky -> king_x, king_y
            if Self::d_in_range(kx + 2, ky + 1) && Self::d_is_enemy_knight(&self, kx + 2, ky + 1) {
                return true;
            }
            if Self::d_in_range(kx + 2, ky - 1) && Self::d_is_enemy_knight(&self, kx + 2, ky - 1) {
                return true;
            }
            if Self::d_in_range(kx + 1, ky + 2) && Self::d_is_enemy_knight(&self, kx + 1, ky + 2) {
                return true;
            }
            if Self::d_in_range(kx + 1, ky - 2) && Self::d_is_enemy_knight(&self, kx + 2, ky - 2) {
                return true;
            }
            // 
            if Self::d_in_range(kx - 1, ky + 1) && Self::d_is_enemy_knight(&self, kx - 1, ky + 1) {
                return true;
            }
            if Self::d_in_range(kx - 1, ky - 1) && Self::d_is_enemy_knight(&self, kx - 1, ky - 1) {
                return true;
            }
            if Self::d_in_range(kx - 2, ky + 2) && Self::d_is_enemy_knight(&self, kx - 2, ky + 2) {
                return true;
            }
            if Self::d_in_range(kx - 2, ky - 2) && Self::d_is_enemy_knight(&self, kx - 2, ky - 2) {
                return true;
            }
            return false;
        }

        fn d_is_enemy_pawn(&self, x: i8, y: i8) -> bool {
            if Self::d_is_enemy(&self, self.board[x as usize][y as usize]) && self.board[x as usize][y as usize].int_representation == 1 {
                return true;
            }
            return false;
        }

        fn d_check_pawn_checks(&self, kx: i8, ky: i8) -> bool {
            let mut dir = 1;
            if self.curr_color.color == 0 {
                dir = -1;
            }
            if Self::d_in_range(kx + dir, ky + 1) && Self::d_is_enemy_pawn(&self, kx + dir, ky + 1) {
                return true;
            }
            if Self::d_in_range(kx + dir, ky - 1) && Self::d_is_enemy_pawn(&self, kx + dir, ky - 1) {
                return true;
            }
            return false;

        }

        pub fn d_in_check(&self) -> bool {

            let mut king_x: i8;
            let mut king_y: i8;

            if (self.t_white) {
                king_x = self.white_king_position[0];
                king_y = self.white_king_position[1];
            } else {
                king_x = self.black_king_position[0];
                king_y = self.black_king_position[1];
            }
            return Self::d_check_scan_diagonals(&self, king_x, king_y);
        }

        pub fn d_make_move(&mut self, mv: Move) {
            self.board[mv.coords[2] as usize][mv.coords[3] as usize] = self.board[mv.coords[0] as usize][mv.coords[1] as usize];
            self.board[mv.coords[0] as usize][mv.coords[1] as usize] = Piece::new(&"00".to_string(), 0, 0);
        }
}

fn main() { 

}
