extern crate image;
extern crate rustc_serialize;
extern crate handlebars;

use self::image::DynamicImage;
use self::rustc_serialize::json::{Json, ToJson};
use self::handlebars::{Handlebars, Context};

use std::path::Path;
use std::fs::File;
use std::collections::BTreeMap;

use errors::SpriteSheetError;
use placement_strategy::PlacementStrategy;
use sprite::{Sprite, PlacedSprite};
use optimization::optimize;



pub struct SpriteSheet {
  name: String,
  canvas: DynamicImage,
  sprites: Vec<PlacedSprite>,
}

impl SpriteSheet {
  pub fn new(sprites: Vec<Sprite>,
             name: &str,
             strategy: PlacementStrategy)
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
    let out_png = out_png.as_ref();
    let out_scss = out_scss.as_ref();

    try!(self.render_scss(out_scss, out_png));

    let mut buffer = try!(File::create(out_png));
    try!(self.canvas.save(&mut buffer, image::ImageFormat::PNG));

    // optimize(out_png);

    Ok(())
  }

  fn render_scss(&self,
                 out_file: &Path,
                 out_img: &Path)
                 -> Result<(), SpriteSheetError> {
    let handlebars = Handlebars::new();
    let template = include_str!("../template.hbs");

    let mut image_path_map: BTreeMap<String, Json> = BTreeMap::new();

    // TODO: Das ist Bullshit. Man sollte vermutlich den relativen Pfad zwischen
    // SCSS-Out und Img-Out ermitteln.
    // let relative_path = &out_img.iter()
    //   .skip_while(|segment| segment.to_str().unwrap() != "images")
    //   .skip(1)
    //   .fold(String::new(), |relative_path, segment| {
    //     relative_path + "/" + segment.to_str().unwrap()
    //   })[1..];
    let relative_path = "./";


    // TODO: WTF! Geht das nicht ein _bisschen_ einfacher?
    image_path_map.insert("image_path".to_owned(), relative_path.to_json());

    let data = Context::wraps(&self.data()).extend(&image_path_map);

    let mut out_file = try!(File::create(out_file));

    // println!("sheet written to {:?}, ARGS: {:?}", out_file, &data);
    try!(handlebars.template_renderw(template, &data, &mut out_file));
    Ok(())
  }

  fn data(&self) -> Json {
    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    m.insert("sprites".to_owned(), self.sprites.to_json());
    m.insert("spritesheet_name".to_owned(), self.name.to_json());
    m.to_json()
  }
}
