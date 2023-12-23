use eframe::egui::{self, Id};
use special_relativity;

pub struct SRApp {
    rng: rand::rngs::ThreadRng,
    c: f32,
}

impl SRApp {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            c: 10.0,
        }
    }
}

impl eframe::App for SRApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("control_panel").show(ctx, |ui| {
            egui::Slider::new(&mut self.c, 1.0..=1000.0).logarithmic(true).text("c");
            ui.separator();
        });

        let id = Id::new("main_view");
        egui::CentralPanel::default().show(ctx, |ui| {
            // compute time step
            let real = ui.input(|i| i.unstable_dt).min(1.0 / 30.0);
        });
    }
}
