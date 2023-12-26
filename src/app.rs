use eframe::{egui::{self, Id, Painter}, egui_glow::painter, epaint::{Pos2, Color32}};
use special_relativity::{self, Spacetime};

pub struct SRApp {
    rng: rand::rngs::ThreadRng,
    spacetime: Spacetime,
    c: f32,
}

impl SRApp {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            spacetime: Spacetime::new(),
            c: 10.0,
        }
    }
}

fn draw(app: &SRApp, painter: &Painter) {
    let r = 20.0;
    let color = Color32::from_rgb(128, 128, 128);
    for frame in app.spacetime.frames.iter() {
        for position in frame.positions.iter() {
            let screen = Pos2::new(position.x, position.y);
            painter.circle_filled(screen, r, color);
        }
    }
}

impl eframe::App for SRApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("control_panel").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.c, 1.0..=1000.0).logarithmic(true).text("c"));
            ui.separator();
        });

        let id = Id::new("main_view");
        egui::CentralPanel::default().show(ctx, |ui| {
            // compute time step
            let real = ui.input(|i| i.unstable_dt).min(1.0 / 30.0);

            draw(self, ui.painter());

        });
    }
}
