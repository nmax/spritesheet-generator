mod bounding_rect;
mod errors;
mod importer;
mod size;
mod sprite;
mod sprite_sheet;
mod template_generator;

use sprite_sheet::SpriteSheet;

extern crate clap;
use clap::{Arg, App, ArgMatches};

fn optional_with_default(matches: &ArgMatches,
                         input: &str,
                         default: &str)
                         -> String {
  matches.value_of(input)
    .map(|s| String::from(s))
    .unwrap_or(String::from(default))
}

// TODO: Verbose Flag mit Logging
// TODO: Args als Enums/Static Strings?
// TODO: Config als Toml File?
fn main() {
  let matches = App::new("SpriteSheetGenerator")
    .version("1.0")
    .author("Maximilian Neger <maximilian.neger@nix-wie-wg.de>")
    .about("Does awesome things")
    .arg(Arg::with_name("input")
      .help("Sets the input file to use")
      .required(true)
      .multiple(true))
    .arg(Arg::with_name("output_scss")
      .long("scss-out")
      .help("Sets the output scss file to use")
      .required(false)
      .takes_value(true))
    .arg(Arg::with_name("output_img")
      .long("image-out")
      .help("Sets the output spritesheet file to use")
      .required(false)
      .takes_value(true))
    .arg(Arg::with_name("name")
      .short("n")
      .long("name")
      .help("Sets the name of the spritesheet. This affects the SASS Mixins \
             and the like.")
      .required(false)
      .takes_value(true))
    .get_matches();

  let out_png =
    optional_with_default(&matches, "output_img", "spritesheet.png");
  let out_scss =
    optional_with_default(&matches, "output_scss", "spritesheet.scss");
  let name = optional_with_default(&matches, "name", "spritesheet");

  println!("[rust] {:?}", out_png);

  // unwrap ist hier sicher weil "input" ein required-Attribut ist
  let files: Vec<&str> = matches.values_of("input").unwrap().collect();

  let sprites = importer::load_files(files).unwrap_or_else(|error| {
    println!("{}", error);
    std::process::exit(1)
  });

  let sheet = SpriteSheet::new(sprites, &name);

  match sheet.save(out_png, out_scss) {
    Err(err) => println!("{}", err),
    Ok(()) => (),
  }
}
