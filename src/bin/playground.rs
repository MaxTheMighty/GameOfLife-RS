use grid::Grid;
use rayon::prelude::*;
use std::time::Instant;
use native_dialog::{FileDialog, MessageDialog, MessageType};
const GRID_SIZE: usize = 50000;
fn main() {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .show_open_single_file()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };

    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();

    if yes {
        println!("{:?}",path);
    }
}
/*
fn main() {
    let mut my_grid: Grid<bool> = Grid::new(GRID_SIZE, GRID_SIZE);
    let mut next_grid: Grid<bool> = Grid::new(GRID_SIZE, GRID_SIZE);
    let starting_time = Instant::now();
    next_grid
        .into_vec()
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, cell)| {
            let (x, y) = (index % GRID_SIZE, index / GRID_SIZE);
            let state = !my_grid[y][x];
            if (state) {
                *cell = true;
            } else {
                *cell = false;
            }
        });
    let stopping_time = Instant::now();
    println!(
        "Difference: {}",
        (stopping_time - starting_time).as_millis()
    );
    /*
    for (index,val) in my_grid.iter_mut().enumerate(){


    }
    */
}
*/
