use eframe::egui;
use egui::{CentralPanel, Color32, Context, Pos2, Rect, Rounding, Stroke, Ui, Vec2, SidePanel};
use native_dialog::{FileDialog};
use std::{env, path::PathBuf};
use graphics::{GameOfLifeRunner, file_parser::FileParser};

const GRID_LENGTH: usize = 200;
const SIDE_PANEL_WIDTH: f32 = 110.0;
const DEFAULT_WINDOW_SIZE: f32 = 1000.0;
const ENTIRE_UI_SIZE_X: f32 = SIDE_PANEL_WIDTH + DEFAULT_WINDOW_SIZE;
const ENTIRE_UI_SIZE_Y: f32 = DEFAULT_WINDOW_SIZE;

fn main() {
    env::set_var("RUST_BACKTRACE", "FULL");
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(
            ENTIRE_UI_SIZE_X,
            ENTIRE_UI_SIZE_Y
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
    game_board: GameOfLifeRunner,
    cells_across_count: usize,
    cells_down_count: usize,
    file_parser: FileParser
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            game_board: GameOfLifeRunner::new(GRID_LENGTH, 10),
            cells_across_count: GRID_LENGTH,
            cells_down_count: GRID_LENGTH,
            cell_width: (DEFAULT_WINDOW_SIZE / GRID_LENGTH as f32),
            file_parser: FileParser::create_empty()//this is very bad 
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let rounding: Rounding = Rounding::none();

       //let (current_width, current_height) = self.get_window_bounds(frame); //Perhaps unnecessary
  
        let side_panel = |ui: &mut Ui| -> (){
            if(ui.add(egui::Button::new("Pause/Unpause")).clicked()){
                self.game_board.invert_running();
            };
            
            if(ui.add(egui::Button::new("Clear")).clicked()){
                self.game_board.stop_running();
                self.game_board.clear();
                self.game_board.start_running();
            }

            if(ui.add(egui::Button::new("Load File")).clicked()){
               let path_opt = MyApp::run_file_dialog();
               let path: PathBuf;
               match path_opt {
                Some(path_valid) => path = path_valid,
                None => {println!("Invalid file"); return},
               }
               println!("{:?}",path);
               self.file_parser.set_file(path);
               self.file_parser.fill_grid(self.game_board.get_board());


            }  
            
        };
        SidePanel::right("Right Panel").default_width(SIDE_PANEL_WIDTH).show(ctx,side_panel);

        let central_panel = |ui: &mut Ui| -> () {
            //self.print_state();
            for y_pos in 0..self.cells_down_count {
                for x_pos in 0..self.cells_across_count {
                    let color: Color32;
                    if self.game_board.get_board().is_alive(x_pos, y_pos) {
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
        CentralPanel::default().show(ctx, central_panel);
        self.game_board.request_update();

        if ctx.input().key_pressed(egui::Key::Space) {
            self.game_board.invert_running();
        }

        if ctx.input().pointer.any_click() {
            self.handle_click(ctx);
        }

        if ctx.input().scroll_delta.y < 0.0 && self.cell_width > 5.0{
            self.cell_width -= 5.0;
            self.cells_across_count = (DEFAULT_WINDOW_SIZE as f32 / self.cell_width).round() as usize;
            self.cells_down_count = (DEFAULT_WINDOW_SIZE / self.cell_width).round() as usize;
        }
        if ctx.input().scroll_delta.y > 0.0 {
            self.cell_width += 5.0;
            self.cells_across_count = (DEFAULT_WINDOW_SIZE  / self.cell_width).round() as usize;
            self.cells_down_count = (DEFAULT_WINDOW_SIZE  / self.cell_width).round() as usize;
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
                if(!self.game_board.get_board().within_bounds(cell_x_pos, cell_y_pos)){
                    return;
                }
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

    fn _print_state(&mut self) {
        println!(
            "[{:?}]\nCell Width: {:?}\nCells Across length: {:?}",
            std::time::SystemTime::now(),
            self.cell_width,
            self.cells_across_count
        );
    }


    fn run_file_dialog() -> Option<PathBuf>{
        let path = FileDialog::new()
        .set_location("~/Desktop")
       // .add_filter("PNG Image", &["png"])
       // .add_filter("JPEG Image", &["jpg", "jpeg"])
        .show_open_single_file()
        .unwrap();
        return path;
    }


}



