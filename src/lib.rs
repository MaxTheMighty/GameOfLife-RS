/*GameOfLifeRunner Organization:
* Variables:
* board: GameOfLife - contains board
* clock: Clock - have to create this class, has Instant and wait period
*
*
 */

pub mod clock;
pub mod file_parser;
pub mod game_of_life;
use clock::Clock;
use file_parser::FileParser;
use game_of_life::GameOfLife;
use native_dialog::FileDialog;
use rayon::prelude::*;
use std::io::Error;
use std::path::{Path, PathBuf};
pub struct GameOfLifeRunner {
    board: game_of_life::GameOfLife,
    clock: clock::Clock,
    running: bool,
    next_board_vec: Vec<bool>,
    multithreading: bool,
    file_parser: FileParser,
}

impl GameOfLifeRunner {
    pub fn default() -> Self {
        Self {
            board: GameOfLife::new(50),
            clock: Clock::new(200),
            running: false,
            next_board_vec: vec![false; 2500],
            multithreading: true,
            file_parser: FileParser::new_empty(),
        }
    }

    pub fn new(game_of_life_bounds: usize, update_interval: u128) -> Self {
        Self {
            board: GameOfLife::new(game_of_life_bounds),
            clock: Clock::new(update_interval),
            running: false,
            next_board_vec: vec![false; game_of_life_bounds * game_of_life_bounds],
            multithreading: true,
            file_parser: FileParser::new_empty(),
        }
    }

    pub fn request_update(&mut self) {
        if self.clock.enough_time_passed() && self.running {
            if self.multithreading {
                self.update_rayon();
            } else {
                self.update();
            }
        }
    }

    pub fn update_rayon(&mut self) {
        self.next_board_vec
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, cell)| {
                let (x, y) = (index % self.board.bounds, index / self.board.bounds);
                let neighbor_count = self.board.neighbor_count(x, y);
                if self.board.is_alive(x, y) {
                    *cell = self.board.cell_lives(neighbor_count);
                } else {
                    *cell = self.board.cell_born(neighbor_count);
                }
            });
        self.board.grid = self.next_board_vec.clone();
    }

    pub fn update(&mut self) {
        self.next_board_vec
            .iter_mut()
            .enumerate()
            .for_each(|(index, cell)| {
                let (x, y) = (index % self.board.bounds, index / self.board.bounds);
                let neighbor_count = self.board.neighbor_count(x, y);
                if self.board.is_alive(x, y) {
                    *cell = self.board.cell_lives(neighbor_count);
                } else {
                    *cell = self.board.cell_born(neighbor_count);
                }
            });
        self.board.grid = self.next_board_vec.clone();
    }

    pub fn run_file_load(&mut self) -> Result<(),Error> {
        let path_opt = Self::open_file_dialog();
        let path: PathBuf;
        match path_opt {
            Some(path_valid) => path = path_valid,
            None => {
                println!("Invalid file");
                return Err()
            } //I dont like this return here, but its
        }
        self.fill_from_file(path)?;
        return Ok(());
    }

    pub fn fill_from_file(&mut self, path: PathBuf) -> Result<(), Error> {
        self.file_parser.set_file(path).unwrap();
        self.file_parser.fill_grid(&mut self.board)?;
        return Ok(());
    }

    pub fn open_file_dialog() -> Result<PathBuf,Error> {
        let path = FileDialog::new()
            .set_location("~/Desktop")
            // .add_filter("PNG Image", &["png"])
            // .add_filter("JPEG Image", &["jpg", "jpeg"])
            .show_open_single_file();

        match path {
            
/*GameOfLifeRunner Organization:
* Variables:
* board: GameOfLife - contains board
* clock: Clock - have to create this class, has Instant and wait period
*
*
 */
        }
    }

}
