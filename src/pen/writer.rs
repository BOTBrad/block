use std::f64::consts::PI;

use svg::node::element::Path;

use pen::Pen;

pub struct Writer {
  pen: Pen,

  vertical_len: f64,
}

impl Writer {
  pub fn new(pen: Pen) -> Self {
    Writer{
      pen: pen,
      vertical_len: 50.0,
    }
  }

  pub fn lozenge(&mut self) -> &mut Self {
    let loz_len = 12.0;
    let loz_ang = PI/4.0;
    self.pen
      .move_by(-loz_len/2.0, loz_ang)
      .by(loz_len, loz_ang)
      .move_by(-loz_len/2.0, loz_ang);

    self
  }

  pub fn vertical(&mut self) -> &mut Self {
    self.vertical_impl(1.0)
  }

  pub fn half_vertical(&mut self) -> &mut Self {
    self.vertical_impl(0.5)
  }

  fn vertical_impl(&mut self, scalar: f64) -> &mut Self {
    self.pen.by(scalar * self.vertical_len, 0.5 * PI);

    self
  }

  pub fn arch(&mut self) -> &mut Self {
    self.pen
      .move_by(0.5 * self.vertical_len, -0.5 * PI)
      .by(0.25 * self.vertical_len, -0.25 * PI)
      .by(0.25 * self.vertical_len, 0.25 * PI);

    self
  }

  pub fn done(self) -> Vec<Path> {
    self.pen.done()
  }
}
