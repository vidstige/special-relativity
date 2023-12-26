use nalgebra::{Vector3, Matrix, U3, ArrayStorage};


// 2D-space time three-vector. x,y and time (z). Time cordinates are stored
// divided by c
type Position = Vector3<f32>; 
type Velocity = Vector3<f32>;

pub struct Frame {
    pub positions: Vec<Position>,
    pub velocity: Velocity,
}

pub struct Spacetime {
    pub metric: Metric,
    pub frames: Vec<Frame>,  // All reference frames
    reference: usize, // The reference in use
}

pub type Matrix3x3<T> = Matrix<T, U3, U3, ArrayStorage<T, 3, 3>>;
type Metric = Matrix3x3<f32>;

const MINKOWSKI: Metric = Metric::new(
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, -1.0,
);

impl Spacetime {
    pub fn new() -> Spacetime {
        let mut frames = Vec::new();
        frames.push(Frame{
            positions: Vec::new(),
            velocity: Velocity::new(0.0, 0.0, 1.0),
        });
        Spacetime {
            metric: MINKOWSKI,
            frames: frames,
            reference: 0,
        }
    }
}

pub fn step(spacetime: &mut Spacetime, dt: f32) {
    for frame in spacetime.frames.iter_mut() {
        for position in frame.positions.iter_mut() {
            *position += frame.velocity * dt;
        }
    }
}
