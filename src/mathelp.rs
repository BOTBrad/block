use matrix::prelude::*;

pub fn rot_mat(angle: f64) -> Conventional<f64> {
  Conventional::from_vec((3, 3), matrix![
    angle.cos(), -angle.sin(), 0.0;
    angle.sin(), angle.cos(), 0.0;
    0.0, 0.0, 1.0;
  ])
}

