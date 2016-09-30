mod bounding_rect;
mod errors;
mod importer;
mod placement_strategy;
mod size;
mod sprite;
mod sprite_sheet;
mod template_generator;

use sprite_sheet::SpriteSheet;
use placement_strategy::PlacementStrategy;

extern crate clap;
use clap::{Arg, App, ArgMatches};

fn optional_with_default(matches: &ArgMatches,
                         input: &str,
                         default: &str)
                         -> String {
  matches.value_of(input)
    .map(String::from)
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
    .arg(Arg::with_name("strategy")
      .long("strategy")
      .short("s")
      .help("Sets the name of the spritesheet. This affects the SASS Mixins \
             and the like.")
      .required(false)
      .takes_value(true))
    .arg(Arg::with_name("verbose")
      .short("v")
      .help("Be more vocal about whats happening")
      .required(false)
      .takes_value(false))
    .get_matches();

  let out_png =
    optional_with_default(&matches, "output_img", "spritesheet.png");
  let out_scss =
    optional_with_default(&matches, "output_scss", "spritesheet.scss");
  let name = optional_with_default(&matches, "name", "spritesheet");
  let strategy = match matches.value_of("strategy") {
    Some("vertical") => PlacementStrategy::StackedVertical,
    Some("horizontal") => PlacementStrategy::StackedHorizontal,
    Some("pack") => PlacementStrategy::Packed,
    _ => PlacementStrategy::Packed,
  };

  println!("{:?}", strategy);

  // unwrap ist hier sicher weil "input" ein required-Attribut ist
  let files: Vec<&str> = matches.values_of("input").unwrap().collect();
  let be_verbose = matches.value_of("verbose").is_some();

  // // TODO: Mehr benutzen / Richtigen Logger benutzen
  // if be_verbose {
  //   println!("[rust] outpng={:?}", out_png);
  //   println!("[rust] outscss={:?}", out_scss);
  //   println!("[rust] files={:?}", files);
  // }

  let sprites = importer::load_files(files).unwrap_or_else(|error| {
    println!("{}", error);
    std::process::exit(1)
  });

  let sheet = SpriteSheet::new(sprites, &name, strategy, be_verbose);
  match sheet.save(out_png, out_scss) {
    Err(err) => println!("{}", err),
    Ok(()) => (),
  }
}
