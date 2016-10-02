extern crate rustc_serialize;

use self::rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct BoundingRect {
  pub x: u32,
  pub y: u32,
  pub width: u32,
  pub height: u32,
}

impl ToJson for BoundingRect {
  fn to_json(&self) -> Json {
    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    m.insert("x".to_owned(), self.x.to_json());
    m.insert("y".to_owned(), self.y.to_json());
    m.insert("width".to_owned(), self.width.to_json());
    m.insert("height".to_owned(), self.height.to_json());
    m.to_json()
  }
}

impl BoundingRect {
  pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
    BoundingRect {
      x: x,
      y: y,
      width: width,
      height: height,
    }
  }
}
