use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Point {
  x: f64,
  y: f64,
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
