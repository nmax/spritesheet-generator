extern crate image;
extern crate rustc_serialize;

use std::path::{Path, PathBuf};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::str::FromStr;

use self::image::{DynamicImage, GenericImage};
use self::rustc_serialize::json::{Json, ToJson};
use size::Size;
use errors::SpriteSheetError;
use bounding_rect::BoundingRect;

pub struct Sprite {
  pub buffer: DynamicImage,
  path: PathBuf,
}

pub struct PlacedSprite {
  pub sprite: Sprite,
  pub position: BoundingRect,
}

impl PlacedSprite {
  pub fn new(sprite: Sprite, position: BoundingRect) -> Self {
    PlacedSprite {
      sprite: sprite,
      position: position,
    }
  }
}

impl ToJson for PlacedSprite {
  fn to_json(&self) -> Json {
    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    m.insert("bounds".to_owned(), self.position.to_json());
    m.insert("name".to_owned(), self.sprite.css_class_name().to_json());
    m.to_json()
  }
}

impl FromStr for Sprite {
  type Err = SpriteSheetError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let image = try!(image::open(&s));

    Ok(Sprite {
      path: Path::new(s).to_path_buf(),
      buffer: image,
    })
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
    let self_width = self.dimensions().0;
    let other_width = other.dimensions().0;

    if self_width == other_width {
      Ordering::Equal
    } else if self_width < other_width {
      Ordering::Greater
    } else {
      Ordering::Less
    }
  }
}

impl Sprite {
  pub fn dimensions(&self) -> Size {
    self.buffer.dimensions()
  }

  // TODO: Das sollte vermutlich "-> Result<String, SpritesheetError>" werden.
  pub fn css_class_name(&self) -> String {
    self.path
      .file_stem()
      .and_then(|os_str| os_str.to_str())
      .map(|s| s.to_lowercase().replace(" ", "_").replace("'", "_"))
      .unwrap_or(String::from("unkown_class_name"))
  }
}
