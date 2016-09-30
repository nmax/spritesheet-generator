extern crate image;
extern crate rustc_serialize;
extern crate oxipng;

use self::image::{GenericImage, DynamicImage};
use self::rustc_serialize::json::{Json, ToJson};

use self::oxipng::{Options as OxiOptions, optimize};

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


    let (width, height) = self.canvas.dimensions();
    // let pixels = self.canvas.raw_pixels();
    // let options = OxiOptions {
    //   backup: false,
    //   out_file: Path::new(&out_png),
    //   out_dir: None,
    //   stdout: false,
    //   fix_errors: false,
    //   pretend: false,
    //   recursive: false,
    //   clobber: true,
    //   create: false,
    //   force: false,
    //   preserve_attrs: false,
    //   verbosity: Some(1),
    //     // filter: HashSet<u8>,
    //     interlace: None
    //     // compression: HashSet<u8>,
    //     // memory: HashSet<u8>,
    //     // strategies: HashSet<u8>,
    //     window: 15u8,
    //     bit_depth_reduction: true,
    //     color_type_reduction: true,
    //     palette_reduction: true,
    //     idat_recoding: true,
    //     strip: None,
    //     use_heuristics: false,
    //     threads: 2
    // };


    let mut buffer = try!(File::create(&out_png));
    try!(self.canvas.save(&mut buffer, image::ImageFormat::PNG));
    match optimize(Path::new(out_png.as_ref()), &OxiOptions::default()) {
      Ok(()) => Ok(()),
      Err(e) => panic!(e),
    }
  }

  pub fn data(&self) -> Json {
    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    m.insert("sprites".to_owned(), self.sprites.to_json());
    m.insert("spritesheet_name".to_owned(), self.name.to_json());
    m.to_json()
  }
}
