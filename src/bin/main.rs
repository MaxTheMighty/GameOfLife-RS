
use eframe::egui;
use egui::{CentralPanel, Color32, Context, Pos2, Rect, Rounding, Stroke, Ui, Vec2};
use std::{
    env,
    time::{Duration, Instant},
};



use grid::*;
pub mod game_of_life_runner;

const GRID_LENGTH: usize = 100;
const DEFAULT_WINDOW_SIZE: usize = 800;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(
            DEFAULT_WINDOW_SIZE as f32,
            DEFAULT_WINDOW_SIZE as f32,
        )),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    cell_width: f32,
    game_board: game_of_life_runner::GameOfLifeRunner,
    cells_across_count: usize,
    cells_down_count: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            game_board: game_of_life_runner::GameOfLifeRunner::new(100, 20),
            cells_across_count: GRID_LENGTH,
            cells_down_count: GRID_LENGTH,
            cell_width: (DEFAULT_WINDOW_SIZE / GRID_LENGTH) as f32,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let rounding: Rounding = Rounding::none();

        let (current_width, current_height) = self.get_window_bounds(frame); //Perhaps unnecessary
        let add_contents = |ui: &mut Ui| -> () {
            //self.print_state();
            for y_pos in 0..self.cells_down_count {
                for x_pos in 0..self.cells_across_count {
                    let color: Color32;
                    if (self.game_board.get_board().is_alive(x_pos, y_pos)) {
                        color = Color32::WHITE;
                    } else {
                        color = Color32::BLACK;
                    }
                    let pos_a: Pos2 = Pos2 {
                        x: (x_pos as f32) * self.cell_width,
                        y: (y_pos as f32) * self.cell_width,
                    };
                    let pos_b: Pos2 = Pos2 {
                        x: (pos_a.x + self.cell_width),
                        y: (pos_a.y + self.cell_width),
                    };

                    ui.painter().rect_stroke(
                        Rect::from_two_pos(pos_a, pos_b),
                        rounding,
                        Stroke::new(1.0, Color32::GRAY),
                    );
                    ui.painter()
                        .rect_filled(Rect::from_two_pos(pos_a, pos_b), rounding, color);
                }
            }
        };

        CentralPanel::default().show(ctx, add_contents);
        self.game_board.request_update();

        if (ctx.input().key_pressed(egui::Key::Space)) {
            self.game_board.invert_running();
        }

        if (ctx.input().pointer.any_click()) {
            self.handle_click(ctx);
        }

        if (ctx.input().scroll_delta.y < 0.0) {
            self.cell_width -= 5.0;
            self.cells_across_count = (current_width / self.cell_width).round() as usize;
            self.cells_down_count = (current_height / self.cell_width).round() as usize;
        }
        if (ctx.input().scroll_delta.y > 0.0) {
            self.cell_width += 5.0;
            self.cells_across_count = (current_width / self.cell_width).round() as usize;
            self.cells_down_count = (current_height / self.cell_width).round() as usize;
        }

        ctx.request_repaint();
    }
}

impl MyApp {
    fn handle_click(&mut self, ctx: &Context) {
        let mouse_pos: Option<Pos2> = ctx.input().pointer.hover_pos();
        match mouse_pos {
            Some(pos) => {
                let x: f32 = pos.x;
                let y = pos.y;

                let cell_x_pos = (x / self.cell_width).floor() as usize;
                let cell_y_pos = (y / self.cell_width).floor() as usize;
                self.game_board
                    .get_board()
                    .invert_cell(cell_x_pos as usize, cell_y_pos as usize);
            }
            None => {}
        }
        println!("{:?}", ctx.input().pointer.hover_pos());
    }

    fn get_window_bounds(&mut self, frame: &eframe::Frame) -> (f32, f32) {
        let window_size = frame.info().window_info.size;
        let (window_size_x, window_size_y) = (window_size.x, window_size.y);
        return (window_size_x, window_size_y);
    }

    fn print_state(&mut self) {
        println!(
            "[{:?}]\nCell Width: {:?}\nCells Across length: {:?}",
            std::time::SystemTime::now(),
            self.cell_width,
            self.cells_across_count
        );
    }
}
