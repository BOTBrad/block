use matrix::prelude::*;
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
      center: Conventional::from_vec((3, 1), matrix![0.0; 0.0; 1.0;]),
    }
  }

  pub fn by(mut self, angle: f64, distance: f64) -> Self {

    /*let x = distance * angle.cos();
    let y = distance * angle.sin() * -1.0;
    let x_w = self.width * self.angle.cos() * angle.cos();
    let y_w = self.height * self.angle.sin() * angle.sin();
    let n = self.next.clone();

    self.parts.push(
      self.style.clone()
        .set("stroke-width", y_w + x_w)
        .set("d", n.clone().line_by((x, y)))
    );

    self.next = n.clone().move_by((x, y));*/

    self
  }

  pub fn done(self) -> Vec<Path> {
    self.parts
  }
}
