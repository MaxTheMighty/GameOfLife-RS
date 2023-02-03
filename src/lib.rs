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
use game_of_life::GameOfLife;
use rayon::prelude::*;
pub struct GameOfLifeRunner {
    board: game_of_life::GameOfLife,
    clock: clock::Clock,
    running: bool,
    next_board_vec: Vec<bool>,
    multithreading: bool,
}

impl GameOfLifeRunner {
    pub fn default() -> Self {
        Self {
            board: GameOfLife::new(50),
            clock: Clock::new(200),
            running: false,
            next_board_vec: vec![false; 2500],
            multithreading: true,
        }
    }

    pub fn new(game_of_life_bounds: usize, update_interval: u128) -> Self {
        Self {
            board: GameOfLife::new(game_of_life_bounds),
            clock: Clock::new(update_interval),
            running: false,
            next_board_vec: vec![false; game_of_life_bounds * game_of_life_bounds],
            multithreading: true,
        }
    }

    pub fn request_update(&mut self) {
        if self.clock.enough_time_passed() && self.running {
            if (self.multithreading) {
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
                if (self.board.is_alive(x, y)) {
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
                if (self.board.is_alive(x, y)) {
                    *cell = self.board.cell_lives(neighbor_count);
                } else {
                    *cell = self.board.cell_born(neighbor_count);
                }
            });
        self.board.grid = self.next_board_vec.clone();
    }

    pub fn clear(&mut self) {
        self.board.grid.fill(false);
    }

    pub fn stop_running(&mut self) {
        self.running = false;
    }

    pub fn start_running(&mut self) {
        self.running = true;
    }

    pub fn running(&self) -> bool {
        return self.running;
    }

    pub fn invert_running(&mut self) {
        self.running = !self.running
    }

    pub fn get_board(&mut self) -> &mut GameOfLife {
        return &mut self.board;
    }

    pub fn set_multithreading(&mut self, value: bool) {
        self.multithreading = value;
    }
}

fn main() {}
