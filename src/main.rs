extern crate svg;

mod pen;

use std::f64::consts::PI;
use svg::Document;
use svg::node::element::Path;

use pen::Pen;

fn main() {
  let p = Pen::new(
    5.0,
    1.0,
    0.15,
    Path::new()
      .set("fill", "none")
      .set("stroke", "black")
  ).by(PI * 0.25, 10.0)
    .by(PI * 0.75, 10.0);

  let mut document = Document::new()
    .set("viewBox", (-100, -100, 200, 200));

  for path in p.done() {
    document = document.add(path);
  }

  svg::save("image.svg", &document).unwrap();
}
