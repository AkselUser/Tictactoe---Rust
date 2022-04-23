use crate::{game::Board, players::{Players, PlayerSymbol}};
use std::io::stdin;



pub fn read_input() -> u32 {
    
    let mut input_line = String::new();
    stdin()
    .read_line(&mut input_line)
    .expect("Failed to read line");

    let trimmed = input_line.trim();
    match trimmed.parse::<u32>() {
        Ok(t) => if t <= 9 {
            t
        } else {
            0
        },
        Err(_e) => 0,
    }
}



pub fn print_board (board: Board, welcome_print: bool) -> Board {
    for i in 0..board.tiles.len() {
        println!("");
        if i != 0 {
            println!("                            -----------------")
        }

        for j in 0..board.tiles[i].len() {
            if j != 0 {
                print!("|")
            } else {
                print!("                            ");
            }
            if welcome_print {
                welcome_print_fn(i, j);
            } else {
                regular_print(&board, i, j);
            }
        }
    }
    println!("\n");
    board
}

fn welcome_print_fn (row: usize, colummn: usize) -> () {
    let calc = 3 * row + colummn + 1;
    print!("  {}", calc.to_string() + "  ")
}

fn regular_print(board: &Board, row: usize, colummn: usize) {
    let x = board.tiles[row][colummn];
    
    match x {
        Some(y) => {
            print!("{}", y);
        }
        None => {print!("     ")},
    };
}