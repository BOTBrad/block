use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

impl Point {
  pub fn new(x: f64, y: f64) -> Point {
    Point{
      x: x,
      y: y,
    }
  }

  pub fn from_dist_angle(dist: f64, angle: f64) -> Point {
    Point{
      x: dist * angle.cos(),
      y: dist * angle.sin(),
    }
  }

  pub fn pair(self) -> (f64, f64) {
    (self.x, self.y)
  }

  pub fn rotate_by(self, angle: f64) -> Self {
    Point{
      x: self.x * angle.cos() - self.y * angle.sin(),
      y: self.x * angle.sin() + self.y * angle.cos(),
    }
  }
}

impl ops::Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point{
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl ops::Sub for Point {
  type Output = Point;

  fn sub(self, other: Point) -> Point {
    Point{
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl ops::Neg for Point {
  type Output = Point;

  fn neg(self) -> Self {
    Point{
      x: -self.x,
      y: -self.y,
    }
  }
}
