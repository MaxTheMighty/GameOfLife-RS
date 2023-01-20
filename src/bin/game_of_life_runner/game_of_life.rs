use std::{
    fmt, sync, thread
};


use sync::{
    Mutex, Arc
};
pub(crate) use firestorm::{
    profile_fn,
    profile_method,
    profile_section
};


pub struct GameOfLife {
    pub bound: usize,
    pub grid: grid::Grid<bool>,
    next_grid: grid::Grid<bool>,
    generation: u32,
    running: bool,
}

impl GameOfLife {
    pub fn new(bounds: usize) -> Self {
        Self {
            bound: bounds,
            grid: grid::Grid::new(bounds as usize, bounds as usize),
            next_grid: grid::Grid::new(bounds as usize, bounds as usize),
            generation: 0,
            running: false,
        }
    }
   
    /* 
    pub fn update_grid(&mut self) {
        let mut next_cell_state;
        for y_pos in 0..self.bound {
            for x_pos in 0..self.bound {
                let neighbor_count = self.neighbor_count(x_pos, y_pos);


                if self.is_alive(x_pos, y_pos) {
                    next_cell_state = self.cell_lives(neighbor_count);

                } else {
                    next_cell_state = self.cell_born(neighbor_count);
                }

                self.next_grid[y_pos][x_pos] = next_cell_state;
            }
        }
        self.grid = self.next_grid.clone();
    }
    */

    pub fn neighbor_count(&self, x_pos_in: usize, y_pos_in: usize) -> u32 {
        
        let mut counter: u32 = 0;
        let bound_as_i32 = self.bound as i32;
        //let neighbors = self.generate_neighbor_indexes(x_pos_in, y_pos_in);
        /*
        for pair in neighbors {
            let x_pos = pair.0;
            let y_pos = pair.1;
            if self.is_alive(x_pos, y_pos) {
                counter += 1;

            }
             */
        
        for x_pos_offset in -1..2{
            for y_pos_offset in -1..2 {
                if x_pos_offset == 0 && y_pos_offset == 0 {
                    continue;
                }

                let mut x_pos_neighbor = x_pos_in as i32 + x_pos_offset;
                let mut y_pos_neighbor = y_pos_in as i32 + y_pos_offset;
                if x_pos_neighbor < 0 {
                    x_pos_neighbor += bound_as_i32;
                }

                if x_pos_neighbor >= bound_as_i32 {
                    x_pos_neighbor -= bound_as_i32;
                }
                if y_pos_neighbor < 0 {
                    y_pos_neighbor += bound_as_i32;
                }
        
                if y_pos_neighbor >= bound_as_i32{
                    y_pos_neighbor -= bound_as_i32;
                }
               if self.is_alive_i32(x_pos_neighbor, y_pos_neighbor) {
                 counter+=1;
               } 
                
            }
        }
        
        



    
        return counter;
    }
    
    pub fn generate_neighbor_indexes(
        &self,
        x_pos_in: usize,
        y_pos_in: usize,
    ) -> Vec<(usize, usize)> {
        //we are given usize, we can convert it into i32 here, get the locations, and then return as usize
        let x_pos_i32 = x_pos_in as i32;
        let y_pos_i32 = y_pos_in as i32;
        let mut coordinates = Vec::new();

        
        
        for x_pos_offset in -1..2 {
            for y_pos_offset in -1..2 {
                if x_pos_offset == 0 && y_pos_offset == 0 {
                    continue;
                }
                let mut x_pos_neighbor = x_pos_i32 + x_pos_offset;
                let mut y_pos_neighbor = y_pos_i32 + y_pos_offset;
                (x_pos_neighbor, y_pos_neighbor) =
                    self.wrap_coordinate_if_needed(x_pos_neighbor, y_pos_neighbor);

                coordinates.push((x_pos_neighbor as usize, y_pos_neighbor as usize));
            }
        }
         

        return coordinates;
    }
    

    
    pub fn wrap_coordinate_if_needed(&self, x_pos_in: i32, y_pos_in: i32) -> (i32, i32) {
        let mut x_pos_out: i32 = x_pos_in;
        let mut y_pos_out: i32 = y_pos_in;
        let bound_as_i32: i32 = self.bound as i32;
        if x_pos_out < 0 {
            x_pos_out += bound_as_i32;
        }
        if x_pos_out >= bound_as_i32 {
            x_pos_out -= bound_as_i32;
        }
        if y_pos_out < 0 {
            y_pos_out += bound_as_i32;
        }

        if y_pos_out >= bound_as_i32 {
            y_pos_out -= bound_as_i32;
        }
        return (x_pos_out, y_pos_out);
    }
 
    pub fn invert_cell(&mut self, x_pos: usize, y_pos: usize) {
        self.grid[y_pos][x_pos] = !self.grid[y_pos][x_pos];
    }

    pub fn is_alive(&self, x_pos: usize, y_pos: usize) -> bool {
        return self.grid[y_pos][x_pos];
    }

    pub fn cell_lives(&self, neighbor_count: u32) -> bool {
        return neighbor_count == 3 || neighbor_count == 2;
    }

    pub fn cell_born(&self, neighbor_count: u32) -> bool {
        return neighbor_count == 3;
    }

    pub fn is_alive_i32(&self, x_pos: i32, y_pos: i32) -> bool{
        return self.grid[y_pos as usize][x_pos as usize];
    }

    pub fn get_grid(&mut self) -> &mut grid::Grid<bool>{
        return & mut self.grid;
    }

    pub fn print(&self) {
        let mut row_out = String::new();
        for y_pos in 0..self.bound {
            for x_pos in 0..self.bound {
                if self.grid[y_pos as usize][x_pos as usize] == true {
                    row_out.push('■')
                } else {
                    row_out.push('_')
                }
            }
            println!("{}", row_out);
            row_out.clear();
        }
    }

    pub fn neighbor_matrix(&self) {
        let mut row_out: String = String::new();
        for y_pos in 0..self.bound {
            for x_pos in 0..self.bound {
                row_out.push(std::char::from_digit(self.neighbor_count(x_pos, y_pos), 10).unwrap());
            }
            println!("{}", row_out);
            row_out.clear();
        }
    }

    pub fn get_running(&self) -> bool {
        return self.running;
    }

    pub fn start_running(&mut self) {
        self.running = true;
    }

    pub fn stop_running(&mut self) {
        self.running = false;
    }

    pub fn set_cell(&mut self, x_pos: usize,y_pos: usize, state:bool){
        self.grid[y_pos][x_pos] = state;
    }
}

impl fmt::Debug for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game of life grid")
            .field("Generation", &self.generation)
            .field("Bounds", &self.bound)
            .field("grid", &self.grid)
            .finish()
    }
}




