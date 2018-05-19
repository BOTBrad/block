extern crate svg;

mod pen;
mod point;

use std::f64::consts::PI;
use svg::Document;
use svg::node::element::Path;

use pen::{Pen, Writer};

fn main() {
  let mut writer = Writer::new(Pen::new(
    10.0,
    2.0,
    -PI/6.0,
    Path::new()
      .set("fill", "none")
      .set("stroke", "black")
  ));

  writer
    .lozenge()
    .vertical()
    .lozenge()
    .arch()
    .half_vertical()
    .lozenge();

  let mut document = Document::new()
    .set("viewBox", (-100, -100, 200, 200));

  for path in writer.done() {
    document = document.add(path);
  }

  svg::save("image.svg", &document).unwrap();
}
