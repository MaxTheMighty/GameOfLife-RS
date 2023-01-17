pub mod game_of_life_runner;


use std::time::{Duration, Instant};

fn main() {
    let board_size: usize = 1000;
    let mut board = game_of_life_runner::game_of_life::GameOfLife::new(board_size); 
    
    board.invert_cell(5,5);
    board.invert_cell(5,6);
    board.invert_cell(4,6);
    board.invert_cell(5,7);
    board.invert_cell(6,5);
    
    let pre_operation = Instant::now();


    for _ in 1..50{
        board.update_board();
    }


    let post_operation = Instant::now();
    let difference = post_operation - pre_operation;
    println!("{:?}",difference.as_millis());
   // board.print();
    //board.neighbor_matrix();

}


