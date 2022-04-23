

use crate::{players::{Players, PlayerSymbol}, in_out};

pub type TileArray = [[Option<PlayerSymbol>; 3]; 3];
pub struct Board {
    pub tiles: TileArray,
}

pub enum GameEnd {
    Win,
    Draw,
    No,
}

// pub fn iterate_tiles<F, G> (board: Board, f: F, g: G) {
//     for i in 0..board.tiles.len() {
//         f(i);
//         for j in 0..board.tiles[i].len() {
//             g(j);
//         }
//     }
// }

impl Board {

    pub fn make_legit_move (mut self, player: &Players) -> (Board, bool) {
        let user_input = in_out::read_input() as f64;
        if user_input == 0.0 {
            println!(" You can only use numbers in range 1..9. Please try again");
            return (self, false);
        }
        let row = ((user_input-1.0)/3.0).floor() as usize;
        let column = ((user_input-1.0)% 3.0) as usize;

        match self.tiles[row][column] {
            Some(_x) => {
                println!("      That tile is already occupied! Try again");
                (self, false)
            }
            None => {
                self.tiles[row][column] = Some(player.symbol);
                (self, true)
            }
        }
    }

    pub fn new () -> Board {
        Board { tiles : [[None; 3]; 3] }
    }


    pub fn check_if_game_is_over(&self, player: &Players) -> GameEnd {
        let mut draw: bool = true;
        for i in 0..self.tiles.len(){
            for j in 0..self.tiles[i].len(){
                if self.tiles[i][j] == None {
                    draw = false;
                }
            }
        }
        if draw { return GameEnd::Draw; }
        Board::check_winning_conditions(&self.tiles, player);

        let mut transpose : TileArray = [[None; 3]; 3];
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                transpose[j][i] = self.tiles[i][j]
            }
        }
        Board::check_winning_conditions(&transpose, player);
        GameEnd::No
    }

    fn check_winning_conditions(board: &TileArray, player: &Players) ->  bool {
        let mut check_hor = true;
        let mut check_diag = true;

        for i in 0..board.len() {
            check_hor = true;
            for j in 0..board[i].len() {
                if board[i][j] != Some(player.symbol) || check_hor != true {
                    check_hor = false;
                }
                if (i == j && board[i][j] != Some(player.symbol)) || check_diag != true {
                    check_diag = false;
                }
            }
        }
        true
    }
}