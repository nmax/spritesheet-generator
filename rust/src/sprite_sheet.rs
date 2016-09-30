extern crate image;
extern crate rustc_serialize;

use self::image::DynamicImage;
use self::rustc_serialize::json::{Json, ToJson};

use std::path::Path;
use std::fs::File;
use std::collections::BTreeMap;

use errors::SpriteSheetError;

use template_generator;
use placement_strategy::*;
use sprite::*;

pub struct SpriteSheet {
  name: String,
  canvas: DynamicImage,
  sprites: Vec<PlacedSprite>,
}

impl SpriteSheet {
  pub fn new(sprites: Vec<Sprite>,
             name: &str,
             strategy: PlacementStrategy,
             be_verbose: bool)
             -> Self {

    let (canvas, placed_sprites) = strategy.place_sprites(sprites);

    SpriteSheet {
      name: name.to_owned(),
      canvas: canvas,
      sprites: placed_sprites,
    }
  }

  pub fn save<P: AsRef<Path>>(&self,
                              out_png: P,
                              out_scss: P)
                              -> Result<(), SpriteSheetError> {
    try!(template_generator::render_scss(&self, &out_scss, &out_png));

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
