use matrix::prelude::*;

pub fn rot(angle: f64) -> Conventional<f64> {
  Conventional::from_vec((3, 3), matrix![
    angle.cos(), -angle.sin(), 0.0;
    angle.sin(), angle.cos(), 0.0;
    0.0, 0.0, 1.0;
  ])
}

pub fn trans(dist: f64, angle: f64) -> Conventional<f64> {
  Conventional::from_vec((3, 3), matrix![
    1.0, 0.0, dist * angle.cos(),
    0.0, 1.0, dist * angle.sin(),
    0.0, 0.0, 1.0;
  ])
}
