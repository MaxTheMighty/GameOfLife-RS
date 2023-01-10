use std::fmt;


pub struct GameOfLife{
    cell_count: u32,
    bound: usize,
    board: grid::Grid<bool>,
    next_board: grid::Grid<bool>,
    generation: u32,
    running: bool

}

impl GameOfLife{
    fn default() -> Self{
        Self {
            cell_count: 25,
            bound: 5,
            board: grid::Grid::new(5,5),
            next_board: grid::Grid::new(5,5),
            generation: 0,
            running: false
        }
    }

    pub fn new(bounds: usize) -> Self {
        Self {
            cell_count: (bounds*bounds) as u32,
            bound: bounds,
            board: grid::Grid::new(bounds,bounds),
            next_board: grid::Grid::new(bounds,bounds),
            generation: 0,
            running: false
        }
    }

    fn neighbor_count(&self, x_pos: usize, y_pos: usize) -> u32 {
        let mut counter: u32 = 0;
        if(x_pos == 0 || x_pos == self.bound-1){
            //request board expansion
            return 0;
        }

        if(y_pos == 0 || y_pos == self.bound-1){
            //request board expansion
            return 0;
        }
        
        for x_pos_index in x_pos-1..x_pos+2{
            for y_pos_index in y_pos-1..y_pos+2{
                if(y_pos_index == y_pos && x_pos_index == x_pos){
                    continue;
                }
              // println!("checking cells: {} and {}",y_pos_index,x_pos_index);
                counter+=(self.board[y_pos_index][x_pos_index] as u32);
            }
        }
       counter
    }

    pub fn invert_cell(&mut self, x_pos: usize, y_pos: usize){
        self.board[y_pos][x_pos] = !self.board[y_pos][x_pos];

    }

    pub fn get_cell(&mut self, x_pos: usize, y_pos: usize) -> bool{
        return self.board[y_pos][x_pos]
    }

    fn print(&self){
        let mut row_out = String::new();
        for y_pos in 0..self.bound{
            
            for x_pos in 0..self.bound{
               if(self.board[y_pos][x_pos] == true){
                row_out.push('■')
               } else {
                row_out.push('_')
               }
               
            }
            println!("{}",row_out);
            row_out.clear();
        }
    }

    pub fn update_board(&mut self){
        let mut neighbor_count = 0;
        let mut next_cell_state = false;
        for y_pos in 0..self.bound{
            for x_pos in 0..self.bound{
                neighbor_count = self.neighbor_count(x_pos, y_pos);
                
                //if alive
                if(self.get_cell(x_pos, y_pos)){
                    if(neighbor_count == 3 || neighbor_count == 2){
                        next_cell_state = true;
                    } else {
                        next_cell_state = false;
                    }
                //dead
                } else {
                    if(neighbor_count == 3){
                        next_cell_state = true;
                    } else {
                        next_cell_state = false;
                    }
                }
                
                self.next_board[y_pos][x_pos] = next_cell_state;
            }
        }
        self.board = self.next_board.clone();
    }


    fn _neighbor_matrix(&self){
        let mut row_out: String = String::new();
        for y_pos in 0..self.bound{
            
            for x_pos in 0..self.bound{
               row_out.push(std::char::from_digit(self.neighbor_count(x_pos, y_pos),10).unwrap());
            
            }
            println!("{}",row_out);
            row_out.clear();
        }
    }


    pub fn get_running(&self) -> bool {
        return self.running;
    }

    pub fn set_running(&mut self) {
        self.running = true;
    }

    pub fn stop_running(&mut self) {
        self.running = false;
    }
    


    
}

impl fmt::Debug for GameOfLife{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game of life board")
        .field("Generation", &self.generation)
        .field("Bounds", &self.bound)
        .field("Board", &self.board).finish()
    }
}


fn main(){
    let mut board: GameOfLife = GameOfLife::new(15);
    board.invert_cell(5,5);
    board.invert_cell(6,6);
    board.invert_cell(7,6);
    board.invert_cell(7,5);
    board.invert_cell(7,4);
    board.print();
    board.update_board();
    board.print();
    board.update_board();
    board.print();
    board.update_board();
    board.print();
    board.update_board();
    board.print();
    board.update_board();
    board.print();
    board.update_board();
    board.print();
    

    //println!("{}",board.neighbor_count(6, 5));
    //board.neighbor_matrix();
  
}