extern crate svg;

mod blockl;
mod pen;
mod point;

use std::f64::consts::PI;
use svg::Document;
use svg::node::element::Path;

use blockl::Fragment as F;
use pen::{FragWriter, Pen, Writer};

fn main() {
  let mut fw = FragWriter::new(10.0, 10.0);
  fw.write(vec![F::Start, F::Descendor, F::Vertical, F::Descendor, F::End]);
  let mut p = Path::new()
    .set("fill", "none")
    .set("stroke", "black")
    .set("d", fw.to_data());
  /*let mut writer = Writer::new(Pen::new(
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
    .lozenge();*/

  let mut document = Document::new()
    .set("viewBox", (-100, -100, 200, 200))
    .add(p);

  /*for path in writer.done() {
    document = document.add(path);
  }*/

  svg::save("image.svg", &document).unwrap();
}
