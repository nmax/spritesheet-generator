extern crate handlebars;

use self::handlebars::{Handlebars, Context};
use std::fs::File;
use std::path::Path;
use sprite_sheet::SpriteSheet;
use errors::SpriteSheetError;

pub fn render_scss<P: AsRef<Path>>(sheet: &SpriteSheet,
                                   out_file: P)
                                   -> Result<(), SpriteSheetError> {
  let handlebars = Handlebars::new();
  let template = include_str!("../template.hbs");
  let data = Context::wraps(&sheet.data());
  let mut out_file = try!(File::create(out_file));

  println!("sheet written to {:?}", out_file);

  try!(handlebars.template_renderw(template, &data, &mut out_file));
  Ok(())
}
