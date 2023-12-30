use eframe::{egui::{self, Id, Painter, Key}, epaint::{Pos2, Color32, Vec2, PathShape, Stroke}};
use nalgebra::{Rotation2, Vector2, Point2};
use special_relativity::{self, Position, Frame, Velocity};

struct Transform {
    
}

impl Transform {
    fn new() -> Transform {
        Transform {}
    }
    fn forward(&self, position: &Point2<f32>) -> Pos2 {
        Pos2::new(position.x, position.y) + Vec2::new(320.0, 200.0)
    }
}

pub struct SRApp {
    rng: rand::rngs::ThreadRng,
    ship: Ship,
    c: f32,
    transform: Transform,
}

struct Ship {
    frame: Frame,
    angle: f32,
}

impl SRApp {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            ship: Ship {
                frame: Frame::new(Position::new(0.0, 0.0, 0.0), Velocity::new(0.0, 0.0, 1.0)),
                angle: 0.0,
            },
            c: 10.0,
            transform: Transform::new(),
        }
    }

    fn control_ship(&mut self, i: &egui::InputState) {
        if i.key_down(Key::ArrowLeft) {
            self.ship.angle -= 0.1;
        }
        if i.key_down(Key::ArrowRight) {
            self.ship.angle += 0.1;
        }
        if i.key_down(Key::ArrowUp) {
            
        }
    }
}


fn draw(app: &SRApp, painter: &Painter, transform: &Transform) {
    ///let r = 20.0;
    let color = Color32::from_rgb(128, 128, 128);
    //let position = app.ship.frame.position;
    //let screen = transform.forward(&position);
    //painter.circle_filled(screen, r, color);
    let points = vec![
        Point2::new(0.0, -10.0),
        Point2::new(5.0, 10.0),
        Point2::new(-5.0, 10.0),
    ];
    let rotation = Rotation2::new(app.ship.angle);
    let ship_shape = PathShape {
        points: points.iter().map(|p| transform.forward(&(rotation * p))).collect(),
        closed: false,
        fill: color,
        stroke: Stroke::NONE,
     };
    painter.add(ship_shape);
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
            draw(self, ui.painter(), &self.transform);
            ui.input(|i| self.control_ship(i));
        });
    }
}
