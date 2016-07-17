extern crate image;
extern crate rustc_serialize;

use std::path::{Path, PathBuf};
use std::cmp::Ordering;
use std::collections::BTreeMap;

use self::image::{GenericImage, DynamicImage};
use self::rustc_serialize::json::{Json, ToJson};
use size::Size;
use errors::SpriteSheetError;
use bounding_rect::BoundingRect;

#[derive(Clone)]
pub struct Sprite {
  path: PathBuf,
  pub buffer: DynamicImage,
  bounds: Option<BoundingRect>,
}

impl ToJson for Sprite {
  fn to_json(&self) -> Json {
    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    if let Some(bounds) = self.bounds.clone() {
      m.insert("bounds".to_owned(), bounds.to_json());
    }
    m.insert("name".to_owned(), self.css_class_name().to_json());
    m.to_json()
  }
}

impl PartialEq for Sprite {
  fn eq(&self, other: &Self) -> bool {
    self.path.eq(&other.path)
  }
}

impl Eq for Sprite {}

impl PartialOrd for Sprite {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.path.partial_cmp(&other.path)
  }
}

impl Ord for Sprite {
  fn cmp(&self, other: &Self) -> Ordering {
    self.path.cmp(&other.path)
  }
}

impl Sprite {
  pub fn new(path: &Path) -> Result<Sprite, SpriteSheetError> {
    let image = try!(image::open(&path));

    Ok(Sprite {
      path: path.to_path_buf(),
      buffer: image,
      bounds: None,
    })
  }

  pub fn dimensions(&self) -> Size {
    self.buffer.dimensions()
  }

  pub fn add_bounds(&mut self, bounds: BoundingRect) {
    self.bounds = Some(bounds);
  }

  pub fn css_class_name(&self) -> String {
    self.path
      .file_stem()
      .and_then(|os_str| os_str.to_str())
      .map(|s| s.to_lowercase().replace(" ", "_"))
      .unwrap_or(String::from("unkown_class_name"))
  }
}
