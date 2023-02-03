use std::fmt;

pub struct GameOfLife {
    pub bounds: usize,
    pub grid: Vec<bool>,

    generation: u32,
    running: bool,
}

impl GameOfLife {
    pub fn new(bounds: usize) -> Self {
        Self {
            bounds: bounds,
            grid: vec![false; bounds * bounds],
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
        let bound_as_i32 = self.bounds as i32;
        for x_pos_offset in -1..2 {
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

                if y_pos_neighbor >= bound_as_i32 {
                    y_pos_neighbor -= bound_as_i32;
                }
                if self.is_alive_i32(x_pos_neighbor, y_pos_neighbor) {
                    counter += 1;
                }
            }
        }
        return counter;
    }

    pub fn wrap_coordinate_if_needed(&self, x_pos_in: i32, y_pos_in: i32) -> (i32, i32) {
        let mut x_pos_out: i32 = x_pos_in;
        let mut y_pos_out: i32 = y_pos_in;
        let bound_as_i32: i32 = self.bounds as i32;
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
        let index = self.convert_index(x_pos, y_pos);
        self.grid[index] = !self.grid[index];
    }

    pub fn is_alive(&self, x_pos: usize, y_pos: usize) -> bool {
        let index = self.convert_index(x_pos, y_pos);
        return self.grid[index];
    }

    pub fn cell_lives(&self, neighbor_count: u32) -> bool {
        return neighbor_count == 3 || neighbor_count == 2;
    }

    pub fn cell_born(&self, neighbor_count: u32) -> bool {
        return neighbor_count == 3;
    }

    pub fn is_alive_i32(&self, x_pos: i32, y_pos: i32) -> bool {
        let index = self.convert_index(x_pos as usize, y_pos as usize);
        return self.grid[index as usize];
    }

    pub fn print(&self) {
        let mut row_out = String::new();
        for y_pos in 0..self.bounds {
            for x_pos in 0..self.bounds {
                let index = self.convert_index(x_pos, y_pos);
                if self.grid[index] == true {
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
        for y_pos in 0..self.bounds {
            for x_pos in 0..self.bounds {
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

    pub fn set_cell(&mut self, x_pos: usize, y_pos: usize, state: bool) {
        let index = self.convert_index(x_pos, y_pos);
        self.grid[index] = state;
    }

    pub fn within_bounds(&self, x_pos: usize, y_pos: usize) -> bool {
        return (x_pos < self.bounds && y_pos < self.bounds);
    }

    fn convert_index(&self, x_pos: usize, y_pos: usize) -> usize {
        if (!self.within_bounds(x_pos, y_pos)) {
            panic!("Index not within bounds!");
        }
        return y_pos * self.bounds + x_pos;
    }
}

impl fmt::Debug for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game of life grid")
            .field("Generation", &self.generation)
            .field("Bounds", &self.bounds)
            .field("grid", &self.grid)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {
        assert_eq!(2 + 2, 4);
    }
}
