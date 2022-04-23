mod game;
use game::*;
mod players;
use crate::{players::{PlayerSymbol, Players}, game::Board, in_out::print_board};
mod in_out;

fn main() -> ! {
    let mut player = Players::new();
    let mut board = Board::new();

    println!("\n\n\n                        *****   TICTACTOE   *****  \n\n
    Welcome to Tictactoe. To play, you simply choose a number to place your symbol.\n
    The tiles each number represents, are printed below.\n
    First to get 3 in a row, wins the game!\n\n
    Coin flip says: '{}' begins", player.symbol);
    
    println!("\n\n\n\n\n\n      Lets begin!\n");

    board = in_out::print_board(board, true);

    loop {
        println!("\n\n      Choose your next move:");
        
        board = in_out::print_board(board, false);

        loop {
            let mut _made_move = false;

            (board, _made_move) = board.make_legit_move(&player);


            board = print_board(board, false);

            if _made_move {
                match board.check_if_game_is_over(&player) {
                    GameEnd::Draw => {println!("      Its a draw!"); std::process::exit(1)},
                    GameEnd::Win => {println!("      {} is the winner!", player.symbol); std::process::exit(1);},
                    GameEnd::No => {}
                }
                if player.symbol == PlayerSymbol::Circle {
                    player.symbol = PlayerSymbol::Cross;
                } else {
                    player.symbol = PlayerSymbol::Circle;
                }
            }
        };
        
    }
}
