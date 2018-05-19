use matrix::prelude::*;
use std::f64::consts::PI;
use std::vec::Vec;
use svg::node::element::Path;
use svg::node::element::path::Data;

use mathelp;

pub struct Pen {
  width: f64,
  height: f64,
  angle: f64,
  style: Path,

  parts: Vec<Path>,
  center: Conventional<f64>,
}

impl Pen {
  pub fn new(w: f64, h: f64, angle: f64, style: Path) -> Self {
    Pen{
      width: w,
      height: h,
      angle: angle,
      style: style,
      parts: Vec::new(),
      center: Conventional::from_vec((1, 3), matrix![
                                     0.0;
                                     0.0;
                                     1.0;]),
    }
  }

  pub fn by(mut self, angle: f64, distance: f64) -> Self {
    let orth = angle + PI*0.5;
    let mut pts = vec![
      mathelp::trans(self.width*0.5, orth).multiply(&self.center),
      mathelp::trans(-self.width*0.5, orth).multiply(&self.center),
    ];
    let t = mathelp::trans(distance, angle);
    let p2 = t.multiply(&pts[1]);
    let p3 = t.multiply(&pts[0]);
    pts.push(p2);
    pts.push(p3);

    self.parts.push(
      self.style.clone()
        .set("stroke-width", 1.0)
        .set("d", Data::new()
          .move_to((pts[0][(0, 0)], pts[0][(1, 0)]))
          .line_to((pts[1][(0, 0)], pts[1][(1, 0)]))
          .line_to((pts[2][(0, 0)], pts[2][(1, 0)]))
          .line_to((pts[3][(0, 0)], pts[3][(1, 0)]))
        )
    );

    self
  }

  pub fn done(self) -> Vec<Path> {
    self.parts
  }
}
