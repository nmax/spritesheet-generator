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
                                   scss_img_url: P)
                                   -> Result<(), SpriteSheetError> {
  let handlebars = Handlebars::new();
  let template = include_str!("../template.hbs");

  let mut image_path_map: BTreeMap<String, Json> = BTreeMap::new();
  let scss_img_url = scss_img_url.as_ref().to_str().unwrap();
  image_path_map.insert("image_path".to_owned(), scss_img_url.to_json());

  let data = Context::wraps(&sheet.data()).extend(&image_path_map);
  let mut out_file = try!(File::create(out_file));

  // println!("sheet written to {:?}, ARGS: {:?}", out_file, &data);

  try!(handlebars.template_renderw(template, &data, &mut out_file));
  Ok(())
}
