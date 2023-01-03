use eframe::{egui, epaint};
use egui::{CentralPanel, Color32, Pos2, Rect, Rounding, Ui, Context};
use rand::Rng;

fn main() {
    let options = eframe::NativeOptions {
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
    cell_states: Vec<bool>,
    last_window_width: f32,
    last_window_height: f32,
    cells_across_count: usize
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            cell_width: 20.0,
            cell_states: vec![false; 10],
            last_window_width: 800.0,
            last_window_height: 600.0,
            cells_across_count: 20
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let rounding: Rounding = Rounding::none();
        //get current window size
        //check if window has been resized
        //if it has, update cells_across_count
        //if not, do not
        let (current_width, current_height) = self.get_window_bounds(frame); //Perhaps unnecessary
        let has_window_resized: bool = self.get_window_resized(current_width,current_height);
        if(has_window_resized){
            
            //get new cells_across_count
            self.cells_across_count = (current_width/self.cell_width).round() as usize;
            println!("Cells across count {}",self.cells_across_count);
            //if this is less than previous, do nothing.
            //if its more, then extend the vector. 
            //This might have to be changed, if they scale up and then down the board will be beyond the screen.
            self.extend_cell_across_if_needed();
        }

       
 


        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            //ui.heading("MY egui application");
            //println!("{:?}",self.cell_states);
            {
                let mut x_pos: usize = 0;
                while(x_pos < self.cells_across_count){
                    let color: Color32;
                    if(self.cell_states[x_pos] == true){
                        color = Color32::WHITE;
                    } else {
                        color = Color32::BLACK;
                    }
                    let pos_a: Pos2 = Pos2 {
                        x: (x_pos as f32) * self.cell_width,
                        y: (0.0),
                    };
                    let pos_b: Pos2 = Pos2 {
                        x: (pos_a.x + self.cell_width),
                        y: (pos_a.y + self.cell_width),
                    };
      
                    ui.painter()
                    .rect_filled(Rect::from_two_pos(pos_a, pos_b), rounding, color);
                    x_pos+=1;
                }
            }
             /*
            ui.painter()
                .rect_filled(Rect::from_two_pos(pos_a, pos_b), rounding, fill_color);

           
            if ui.button("Click me").clicked() {
                self.randomize_colors();
                fill_color = Color32::from_rgb(self.r, self.g, self.b);
            };
 */         
            if (ctx.input().pointer.any_click()) {
                self.handle_click(ctx);
            }
        });
    }
}

impl MyApp {
    /*
    fn randomize_colors(&mut self) {
        let mut rng = rand::thread_rng();
        self.r = rng.gen_range(0..255);
        self.g = rng.gen_range(0..255);
        self.b = rng.gen_range(0..255);
    }
     */


    fn handle_click(&mut self, ctx: &Context){
        let mouse_pos: Option<Pos2> = ctx.input().pointer.hover_pos();
        match mouse_pos {
            Some(pos) => {
                let x: f32 = pos.x;
                let y = pos.y;

                let mut cell_pos = (x/self.cell_width).floor() as usize;
                self.cell_states[cell_pos] = !self.cell_states[cell_pos];
                
            }

            None => {}
        }
        println!("{:?}", ctx.input().pointer.hover_pos());
    }

    
    fn get_window_bounds(&mut self, frame: &eframe::Frame) -> (f32,f32){
        let window_size = frame.info().window_info.size;
        let (window_size_x,window_size_y) = (window_size.x,window_size.y);
        return (window_size_x,window_size_y)
        
    }

    fn get_window_resized(&mut self, current_width: f32, current_height: f32) -> bool{
        let flag: bool = (current_width != self.last_window_width) || (current_height != self.last_window_height);
        //println!("Comparing current width: {} to last width: {}", current_width,self.last_window_width);
        if flag  {
            self.last_window_width = current_width;
            self.last_window_height = current_height;
        }
        return flag
    }

    fn extend_cell_across_if_needed(&mut self) -> bool{
        //println!("Cells across count: {}",self.cells_across_count);
        if(self.cells_across_count > self.cell_states.len()){

            self.cell_states.resize(self.cells_across_count,false);
            println!("[{:?}] Cells have been resized to len {}",std::time::SystemTime::now(),self.cell_states.len());
            return true
        }
        return false
    }

    
    


}
