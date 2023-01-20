pub mod game_of_life_runner;


use std::time::{Duration, Instant};

use game_of_life_runner::GameOfLifeRunner;
const board_size: usize = 10000;
fn main() {
   
    
    
    let pre_operation = Instant::now();

    //firestorm::bench("./flames/", || update_50_times()).unwrap();


    update_50_times();
    let post_operation = Instant::now();
    let difference = post_operation - pre_operation;
    println!("{:?}",difference.as_millis());
   // board.print();
    //board.neighbor_matrix();

}

fn update_50_times(){
    let mut board = GameOfLifeRunner::new(board_size,0);
    
    board.get_board().invert_cell(5,5);
    board.get_board().invert_cell(5,6);
    board.get_board().invert_cell(4,6);
    board.get_board().invert_cell(5,7);
    board.get_board().invert_cell(6,5);
    for _ in 1..50{
        board.update();
    }
}

