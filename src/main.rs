use eframe::{egui, epaint};
use egui::{CentralPanel, Color32, Pos2, Rect, Rounding, Ui};
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
    counter: i32,
    r: u8,
    g: u8,
    b: u8,
    cell_width: f32,
    cell_states: Vec<bool>
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            counter: 0,
            r: 0,
            g: 100,
            b: 100,
            cell_width: 20.0,
            cell_states: vec![false; 10]
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let rounding: Rounding = Rounding {
            nw: (0.0),
            ne: (0.0),
            sw: (0.0),
            se: (0.0),
        };

       
        let mut fill_color: Color32 = Color32::from_rgb(self.r, self.g, self.b);
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            //ui.heading("MY egui application");
            println!("{:?}",self.cell_states);
            {
                let mut counter: usize = 0;
                while(counter < 10){
                    let color: Color32;
                    if(self.cell_states[counter] == true){
                        color = Color32::WHITE;
                    } else {
                        color = Color32::BLACK;
                    }
                    let pos_a: Pos2 = Pos2 {
                        x: (counter as f32) * self.cell_width,
                        y: (0.0),
                    };
                    let pos_b: Pos2 = Pos2 {
                        x: (pos_a.x + self.cell_width),
                        y: (pos_a.y + self.cell_width),
                    };
      
                    ui.painter()
                    .rect_filled(Rect::from_two_pos(pos_a, pos_b), rounding, color);
                    counter+=1;
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
        });
    }
}

impl MyApp {
    fn randomize_colors(&mut self) {
        let mut rng = rand::thread_rng();
        self.r = rng.gen_range(0..255);
        self.g = rng.gen_range(0..255);
        self.b = rng.gen_range(0..255);
    }
}
