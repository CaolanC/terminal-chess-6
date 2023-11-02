#![allow(unused)]

mod piece;

struct Board
{
    board: [piece::Piece; 8],
}

impl Board {

    pub fn new_empty() -> Self {

        Self {board: [piece::Piece {
            is_white: false,
            is_black: false,
            is_empty: true,
            int_representation: 0,

       
        }; 8]}
    }

    pub fn default_fill(&self){
        for i in 0..8 {
            for j in 0..8
            {
                println!("{} - {}", i.to_string(), j.to_string());
            };
        };
    }
}

fn main()
{
    let def = Board::new_empty();
    println!("{}", def.board[0].int_representation.to_string());
    def.default_fill();
}
