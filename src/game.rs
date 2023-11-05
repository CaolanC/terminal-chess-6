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


    pub struct Board
    {
        board: [[Piece; 8]; 8],
    }

    impl Board {

        pub fn new_empty() -> Self {

            Self {board: [[Piece {
                is_white: false,
                is_black: false,
                is_empty: true,
                int_representation: 0,   
            }; 8]; 8]}
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

fn main() { 
    println!("Game ran without issue.");
}
