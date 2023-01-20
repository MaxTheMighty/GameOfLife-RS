use grid::Grid;
use std::time::{Instant};
use rayon::prelude::*;
const GRID_SIZE: usize = 50000;
fn main(){
    let mut my_grid: Grid<bool> = Grid::new(GRID_SIZE,GRID_SIZE);
    let mut next_grid: Grid<bool> = Grid::new(GRID_SIZE,GRID_SIZE);
    let starting_time = Instant::now();
    next_grid.into_vec().par_iter_mut().enumerate().for_each(|(index,cell)|{
        let(x,y) = (index%GRID_SIZE,index/GRID_SIZE); 
        let state = !my_grid[y][x];
        if(state){
          *cell = true;
        } else {
          *cell = false;
        }
    });
    let stopping_time = Instant::now();
    println!("Difference: {}",(stopping_time-starting_time).as_millis());
    /* 
    for (index,val) in my_grid.iter_mut().enumerate(){
        

    }
    */
}