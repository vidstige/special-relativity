use eframe::{egui::{self, Id, Painter, Key}, epaint::{Pos2, Color32, Vec2, PathShape, Stroke}};
use nalgebra::{Rotation2, Vector2, Point2};
use special_relativity::{self, Position, Frame, Velocity, Observer};

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
    observer: Observer,
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
            observer: Observer{velocity: Velocity::new(0.0, 0.0, 1.0)},
            ship: Ship {
                frame: Frame::new(Position::new(0.0, 0.0, 0.0), Velocity::new(0.0, 0.0, 1.0)),
                angle: 0.0,
            },
            c: 10.0,
            transform: Transform::new(),
        }
    }

    fn control_ship(&mut self, i: &egui::InputState, dt: f32) {
        if i.key_down(Key::ArrowLeft) {
            self.ship.angle -= 0.2;
        }
        if i.key_down(Key::ArrowRight) {
            self.ship.angle += 0.2;
        }
        if i.key_down(Key::ArrowUp) {
            let a = Rotation2::new(self.ship.angle) * Vector2::new(0.0, -20.0);
            self.ship.frame.velocity += Velocity::new(a.x, a.y, 0.0) * dt;
        }
    }
}

fn draw_grid(painter: &Painter, size: Vec2) {
    let stroke = Stroke::new(1.0, Color32::from_rgb(64, 64, 64));
    let spacing = 32.0;
    let width = (size.x / spacing) as usize;
    for x in 0..width {
        painter.vline(x as f32 * spacing, 0.0..=size.y, stroke);
    }
    let height = (size.y / spacing) as usize;
    for y in 0..height {
        painter.hline(0.0..=size.x, y as f32 * spacing, stroke);
    }
}

fn draw_ship(painter: &Painter, app: &SRApp) {
    let color = Color32::from_rgb(16, 160, 128);
    let points = vec![
        Point2::new(0.0, -1.5) * 15.0,
        Point2::new(1.0, 1.5) * 15.0,
        Point2::new(-1.0, 1.5) * 15.0,
    ];
    let rotation = Rotation2::new(app.ship.angle);
    
    let delta = Vector2::new(app.ship.frame.position.x, app.ship.frame.position.y);
    let ship_shape = PathShape {
        points: points.iter().map(|p| app.transform.forward(&(rotation * p + delta))).collect(),
        closed: true,
        fill: color,
        stroke: Stroke::NONE,
     };
    painter.add(ship_shape);
}

fn draw(painter: &Painter, size: Vec2, app: &SRApp) {
    draw_grid(painter, size);
    draw_ship(painter, app);
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

            draw(ui.painter(), ui.available_size(), self);
            ui.input(|i| self.control_ship(i, real));
            self.ship.frame.position += self.ship.frame.velocity * real;
            //println!("{:?} {:?}", self.ship.frame.velocity, self.ship.frame.position);

            ui.ctx().request_repaint();
        });
    }
}
