use nalgebra::{Vector3, Matrix, U3, ArrayStorage};


// 2D-space time three-vector. x,y and time (z). Time cordinates are stored
// divided by c
pub type Position = Vector3<f32>; 
pub type Velocity = Vector3<f32>;

pub struct Frame {
    pub position: Position,
    pub velocity: Velocity,
}

impl Frame {
    pub fn new(position: Position, velocity: Velocity) -> Frame {
        Frame {position, velocity }
    }
}

pub type Matrix3x3<T> = Matrix<T, U3, U3, ArrayStorage<T, 3, 3>>;
type Metric = Matrix3x3<f32>;

const MINKOWSKI: Metric = Metric::new(
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, -1.0,
);
