use std::f64::consts::PI;
use std::vec::Vec;

use svg::node::element::path::Data;

use blockl::Fragment;
use pen::Pen;
use point::Point;

pub struct FragWriter {
  width: f64,
  height: f64,

  path: Vec<(f64, f64)>,
}

impl FragWriter {
  pub fn new(width: f64, height: f64) -> FragWriter {
    FragWriter{
      width: width,
      height: height,

      path: Vec::new(),
    }
  }

  pub fn write(&mut self, frags: Vec<Fragment>) -> &mut Self {
    for i in 0..frags.len()-1 {
      let c = &frags[i];
      let n = &frags[i+1];
      let (f, _) = self.get_segments(c, n);

      self.add_seg(f);
    }

    self
  }

  fn get_segments(&self, f1: &Fragment, f2: &Fragment) -> ((f64, f64), (f64, f64)) {
    match (self.get_frag(f1), self.get_frag(f2)) {
      (None, Some(b)) =>
        (b, b),
      (Some(a), Some(b)) =>
        (a, b),
      (Some(a), None) =>
        (a, a),
      _ =>
        unreachable!(),
    }
  }

  fn get_frag(&self, f: &Fragment) -> Option<(f64, f64)> {
    match f {
      Fragment::Start => 
        None,
      Fragment::Descendor =>
        Some((8.0, PI/6.0)),
      Fragment::Vertical =>
        Some((50.0, PI/2.0)),
      Fragment::End =>
        None,
    }
  }

  fn add_seg(&mut self, off: (f64, f64)) {
    self.path.push(off);
  }

  pub fn to_data(self) -> Data {
    let mut data = Data::new();
    let mut center = Point::new(0.0, 0.0);
    data = data.move_to(center.pair());

    for (dist, angle) in self.path {
      center = center + Point::from_dist_angle(dist, angle);
      data = data.line_to(center.pair());
    }

    data
  }
}
