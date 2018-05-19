use std::vec::Vec;

use self::pen::Pen;

use blockl::{self, Fragment};

impl blockl::Writer for Pen {
  pub fn write(mut self, fragments Vec<Fragment>) -> self {
    for f in fragments {
      f.

