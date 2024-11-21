mod game;
mod board;
mod ship;

use crate::board::Board;
//static mut grid : [[char;10];10] = [['I'; 10]; 10];
// static mut targets : [[i32; 17]; 2] = [[-1; 17]; 2];
// static mut target_counter : i32 = 0;


fn main() {

    println!("Loading...");

    //hv to initialize the grid here
    let board = Board::new();
    board.print_board();

    
}
