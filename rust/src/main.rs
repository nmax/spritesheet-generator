mod bounding_rect;
mod errors;
mod optimization;
mod placement_strategy;
mod size;
mod sprite;
mod sprite_sheet;

use sprite::Sprite;
use sprite_sheet::SpriteSheet;
use placement_strategy::PlacementStrategy;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

struct Arguments {
  sprites: Vec<Sprite>,
  name: String,
  out_png: String,
  out_scss: String,
  strategy: PlacementStrategy,
  verbosity: Option<u8>,
}

fn parse_args() -> Arguments {
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

  Arguments {
    name: value_t!(matches, "name", String).unwrap_or(format!("spritesheet")),
    sprites: values_t!(matches, "input", Sprite).unwrap(),
    out_scss: value_t!(matches, "output_scss", String)
      .unwrap_or(format!("spritesheet.scss")),
    out_png: value_t!(matches, "output_img", String)
      .unwrap_or(format!("spritesheet.png")),

    strategy: match matches.value_of("strategy") {
      Some("vertical") => PlacementStrategy::StackedVertical,
      Some("horizontal") => PlacementStrategy::StackedHorizontal,
      Some("pack") => PlacementStrategy::Packed,
      _ => PlacementStrategy::Packed,
    },
    verbosity: match matches.occurrences_of("v") {
      0 => None,
      1 => Some(1),
      2 => Some(2),
      _ => Some(2),
    },
  }
}

// TODO: Verbose Flag mit Logging
// TODO: Config als Toml File?
fn main() {
  let arguments = parse_args();

  // // TODO: Mehr benutzen / Richtigen Logger benutzen
  // if be_verbose {
  //   println!("[rust] outpng={:?}", out_png);
  //   println!("[rust] outscss={:?}", out_scss);
  //   println!("[rust] files={:?}", files);
  // }

  let sheet = SpriteSheet::new(arguments.sprites,
                               &arguments.name,
                               arguments.strategy,
                               arguments.verbosity);

  match sheet.save(arguments.out_png, arguments.out_scss) {
    Err(err) => println!("{}", err),
    Ok(()) => (),
  }
}
