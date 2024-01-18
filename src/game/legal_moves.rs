mod game;
use game::Board;
impl Board
{

    pub fn scan_lines(&self, _x: usize, _y: usize, mut legal_moves: Vec<[i8;2]>) -> Vec<[i8; 2]> {
        for x in 0.._x {
            for y in 0.._y {
            }
        }
        legal_moves.push([1;2]);
        return legal_moves;
    }
}

fn main() {
    let x = Board::default_new();
    let y = Vec::<[i8; 2]>::new();
    x.scan_lines(0, 0, y);
}
