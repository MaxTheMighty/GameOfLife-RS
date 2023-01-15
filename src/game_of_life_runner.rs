/*GameOfLifeRunner Organization:
* Variables:
* board: GameOfLife - contains board
* clock: Clock - have to create this class, has Instant and wait period
*
*
 */



pub mod clock;
pub mod game_of_life;

use game_of_life::GameOfLife;
use clock::Clock;

pub struct GameOfLifeRunner {
    board: game_of_life::GameOfLife,
    clock: clock::Clock,
    running: bool,
}

impl GameOfLifeRunner {
    pub fn default() -> Self {
        Self {
            board:GameOfLife::new(50),
            clock: Clock::new(200),
            running: false,
        }
    }

    pub fn new(game_of_life_bounds: usize, update_interval: u128) -> Self {
        Self {
            board: GameOfLife::new(game_of_life_bounds),
            clock: Clock::new(update_interval),
            running: false,
        }
    }

    pub fn request_update(&mut self) {
        if self.clock.enough_time_passed() && self.running {
            self.board.update_board();
        }
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
}

fn main() {
    let mut runner: GameOfLifeRunner = GameOfLifeRunner::default();
    runner.board.print();
    runner.request_update();
    runner.board.print()
}
