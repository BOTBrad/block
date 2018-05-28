use std::f64;
use std::f64::consts::PI;
use std::vec::Vec;

use svg::node::element::path::Data;

use blockl::Fragment;
use pen::Pen;
use point::Point;

pub struct FragWriter {
  width: f64,
  height: f64,

  center: Point,
  left: Vec<Point>,
  right: Vec<Point>,
}

impl FragWriter {
  pub fn new(width: f64, height: f64) -> FragWriter {
    FragWriter{
      width: width,
      height: height,

      center: Point::new(0.0, 0.0),
      left: Vec::new(),
      right: Vec::new(),
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
    let (d, a) = off;
    self.center = self.center + Point::from_dist_angle(d, a);
    let pt = self.center;
    println!("{:?}", pt);
    let (left, right) = self.thickness_offset(a);
    self.left.push(pt + left);
    self.right.push(pt + right);
  }

  fn thickness_offset(&self, angle: f64) -> (Point, Point) {
    let orth = angle - PI / 2.0;

    let (max, min) = thickness_rotate(
      vec![
        Point::new(self.width, self.height),
        Point::new(self.width, -self.height),
        Point::new(-self.width, self.height),
        Point::new(-self.width, -self.height),
      ],
      -angle,
    );

    (
      Point::from_dist_angle(max * 5.0, orth),
      Point::from_dist_angle(min * 5.0, orth),
    )
  }

  pub fn to_data(self) -> Data {
    let mut data = Data::new();
    data = data.move_to((0.0, 0.0));
    for pt in self.left {
      data = data.line_to(pt.pair());
    }

    data = data.move_to((0.0, 0.0));
    for pt in self.right {
      data = data.line_to(pt.pair());
    }

    data
  }
}

fn thickness_rotate(pts: Vec<Point>, angle: f64) -> (f64, f64) {
  if pts.len() == 0 {
    panic!();
  }

  (-1.0, 1.0)

  /*let mut max = f64::MIN;
  let mut min = f64::MIN;

  for pt in pts {
    let p = pt.rotate_by(angle);

    if p.y > max {
      max = p.y;
    }

    if p.y < min {
      min = p.y;
    }
  }

  (max, min)*/
}

