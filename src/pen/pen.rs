use std::f64::consts::PI;
use std::vec::Vec;
use svg::node::element::Path;
use svg::node::element::path::Data;

use point::Point;

pub struct Pen {
  width: f64,
  height: f64,
  angle: f64,
  style: Path,

  parts: Vec<Path>,
  center: Point,
}

impl Pen {
  pub fn new(w: f64, h: f64, angle: f64, style: Path) -> Self {
    Pen{
      width: w,
      height: h,
      angle: angle,
      style: style,
      parts: Vec::new(),
      center: Point::new(0.0, 0.0),
    }
  }

  pub fn by(mut self, angle: f64, distance: f64) -> Self {
    let stroke = Point::from_dist_angle(distance, angle);
    let mut parts_vec = self.get_starting_pairs();

    for part in &mut parts_vec {
      let p2 = part[1] + stroke;
      let p3 = part[0] + stroke;

      part.push(p2);
      part.push(p3);
    }

    for p in parts_vec {
      self.parts.push(
        self.style.clone()
          .set("fill", "black")
          .set("stroke", "none")
          .set("d", Data::new()
            .move_to(p[0].pair())
            .line_to(p[1].pair())
            .line_to(p[2].pair())
            .line_to(p[3].pair())
          )
      );
    }

    self.center = self.center + stroke;
    self
  }

  fn get_starting_pairs(&self) -> Vec<Vec<Point>> {
    let brush_w = Point::from_dist_angle(self.width, self.angle);
    let brush_h = Point::from_dist_angle(self.height, self.angle + 0.5 * PI);
    let c = self.center;

    vec![
      vec![c + brush_w + brush_h, c - brush_w + brush_h],
      vec![c + brush_w - brush_h, c - brush_w - brush_h],
      vec![c + brush_w + brush_h, c + brush_w - brush_h],
      vec![c - brush_w + brush_h, c - brush_w - brush_h],
    ]
  }

  pub fn done(self) -> Vec<Path> {
    self.parts
  }
}
