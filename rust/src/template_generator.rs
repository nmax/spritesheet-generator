extern crate handlebars;
extern crate rustc_serialize;

use self::handlebars::{Handlebars, Context};
use self::rustc_serialize::json::{Json, ToJson};
use std::fs::File;
use std::path::Path;
use sprite_sheet::SpriteSheet;
use errors::SpriteSheetError;
use std::collections::BTreeMap;

pub fn render_scss<P: AsRef<Path>>(sheet: &SpriteSheet,
                                   out_file: P,
                                   out_img: P)
                                   -> Result<(), SpriteSheetError> {
  let handlebars = Handlebars::new();
  let template = include_str!("../template.hbs");

  let mut image_path_map: BTreeMap<String, Json> = BTreeMap::new();

  // TODO: Das ist Bullshit. Man sollte vermutlich den relativen Pfad zwischen
  // SCSS-Out und Img-Out ermitteln.
  // let relative_path = &out_img.as_ref()
  //   .iter()
  //   .skip_while(|segment| segment.to_str().unwrap() != "images")
  //   .skip(1)
  //   .fold(String::new(), |relative_path, segment| {
  //     relative_path + "/" + segment.to_str().unwrap()
  //   })[1..];
  let relative_path = "./";


  // TODO: WTF! Geht das nicht ein _bisschen_ einfacher?
  image_path_map.insert("image_path".to_owned(), relative_path.to_json());

  let data = Context::wraps(&sheet.data()).extend(&image_path_map);

  let mut out_file = try!(File::create(out_file));

  // println!("sheet written to {:?}, ARGS: {:?}", out_file, &data);

  try!(handlebars.template_renderw(template, &data, &mut out_file));
  Ok(())
}
