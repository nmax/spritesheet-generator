extern crate image;
extern crate rustc_serialize;

use self::image::{DynamicImage, GenericImage, imageops};
use self::rustc_serialize::json::{Json, ToJson};

use std::path::Path;
use std::fs::File;
use std::collections::BTreeMap;

use errors::SpriteSheetError;
use template_generator;
use size::Size;
use sprite::Sprite;
use bounding_rect::BoundingRect;

pub struct SpriteSheet {
  name: String,
  canvas: DynamicImage,
  sprites: Vec<Sprite>,
  be_verbose: bool,
}

impl SpriteSheet {
  // TODO: Kann das weniger machen? Lieber ein kleineres Struct und dickere
  // Methoden?
  pub fn new(mut sprites: Vec<Sprite>, name: &str, be_verbose: bool) -> Self {
    let (width, height) = compute_min_spritesheet_size(sprites.clone());
    let mut canvas = DynamicImage::new_rgba8(width, height);
    let mut offset: u32 = 0;

    for sprite in &mut sprites {
      let bounds = place_image(sprite, &mut canvas, offset);
      sprite.add_bounds(bounds);
      offset += sprite.dimensions().1 + 1;
    }

    SpriteSheet {
      name: name.to_owned(),
      canvas: canvas,
      sprites: sprites,
      be_verbose: be_verbose,
    }
  }

  pub fn save<P: AsRef<Path>>(&self,
                              out_png: P,
                              out_scss: P)
                              -> Result<(), SpriteSheetError> {
    try!(template_generator::render_scss(self, &out_scss, &out_png));

    let mut buffer = try!(File::create(&out_png));
    try!(self.canvas.save(&mut buffer, image::ImageFormat::PNG));
    Ok(())
  }

  pub fn data(&self) -> Json {
    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    m.insert("sprites".to_owned(), self.sprites.to_json());
    m.insert("spritesheet_name".to_owned(), self.name.to_json());
    m.to_json()
  }
}

fn place_image(sprite: &Sprite,
               buffer: &mut DynamicImage,
               offset: u32)
               -> BoundingRect {
  let (width, height) = sprite.dimensions();
  let mut sub_image = imageops::crop(buffer, 0, offset, width, height);
  sub_image.copy_from(&sprite.buffer, 0, 0);

  BoundingRect {
    x: 0,
    y: offset,
    width: width,
    height: height,
  }
}

fn compute_min_spritesheet_size(sprites: Vec<Sprite>) -> Size {
  let mut min_width = 0u32;
  let mut min_height = 0u32;

  for sprite in sprites {
    let (width, height) = sprite.dimensions();
    min_height += height + 1;

    if width > min_width {
      min_width = width
    }
  }

  (min_width, min_height)
}
