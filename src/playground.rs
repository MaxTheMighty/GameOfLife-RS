mod game_of_life;
use std::time::{Duration, Instant};

fn main() {
    let mut board = game_of_life::GameOfLife::new(10, 10);
    board.invert_cell(0, 0);
    let vec = board.generate_neighbor_indexes(0, 0);
    for pair in vec {
        println!("{:?}", pair);
    }
    board.print();
    board.neighbor_matrix();
}

/*fn neighbor_count_debug(x_pos: usize, y_pos: usize, board_size: i32){
    for x_pos_index in x_pos-1..x_pos+2{
        for y_pos_index in y_pos-1..y_pos+2{

            let mut y_cell_index = y_pos_index;
            let mut x_cell_index = x_pos_index;



            print!("{:>2} {} ",x_cell_index,y_cell_index);
          // println!("checking cells: {} and {}",y_pos_index,x_pos_index);
            //counter+=(self.board[y_pos_index][x_pos_index] as u32);
        }
        println!(" ");
    }
}
*/
