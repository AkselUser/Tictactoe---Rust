

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
        if Board::check_winning_conditions(&self.tiles, player) {
            return GameEnd::Win;
        }

        GameEnd::No
    }

    fn check_winning_conditions(board: &TileArray, player: &Players) ->  bool {
        let mut is_win = false;
        let mut transpose : TileArray = [[None; 3]; 3];
        println!("{}", player.symbol);
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                transpose[j][i] = board[i][j];
            }
        }

        for i in 0..board.len() {
            if board[i] == [Some(player.symbol); 3] || transpose[i] == [Some(player.symbol); 3] {is_win = true;}
        }

        if board[0][0] == Some(player.symbol) && board[1][1] == Some(player.symbol) && board[2][2] == Some(player.symbol) {is_win = true;}

        is_win
    }
}