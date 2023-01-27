pub mod game_of_life_runner;

use std::env;
use std::time::{Duration, Instant};

use game_of_life_runner::game_of_life::GameOfLife;
const BOARD_SIZE: usize = 10000;
fn main() {
   
    
    
    let pre_operation = Instant::now();
    update_50_times();
    let post_operation = Instant::now();
    let difference = post_operation - pre_operation;
    println!("{:?}", difference.as_millis());
}

fn update_50_times() {
    let mut board = game_of_life_runner::GameOfLifeRunner::new(BOARD_SIZE, 50);

    board.get_board().invert_cell(5, 5);
    board.get_board().invert_cell(5, 6);
    board.get_board().invert_cell(4, 6);
    board.get_board().invert_cell(5, 7);
    board.get_board().invert_cell(6, 5);
    for _ in 1..50 {
        board.update_rayon();
    }
}

fn print_test() {
    let mut board = game_of_life_runner::GameOfLifeRunner::new(BOARD_SIZE, 50);
    board.get_board().invert_cell(5, 5);
    board.get_board().invert_cell(5, 6);
    board.get_board().invert_cell(4, 6);
    board.get_board().invert_cell(5, 7);
    board.get_board().invert_cell(6, 5);
    board.get_board().print();
    board.update();
    board.get_board().print();
}
