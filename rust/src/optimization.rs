extern crate oxipng;

use self::oxipng::{Options as OxiOptions, optimize as oxi_optimize};
use self::oxipng::headers::Headers;

use std::collections::HashSet;
use std::path::{Path, PathBuf};

extern crate log;
use log::{max_log_level, LogLevelFilter};

pub fn optimize(img_path: &Path) {
  let opts = oxi_options(img_path.to_path_buf());

  match oxi_optimize(img_path, &opts) {
    Ok(()) => (),
    Err(e) => println!("{}", e),
  }
}

// https://shssoichiro.github.io/oxipng/doc/oxipng/struct.Options.html
fn oxi_options(out_png: PathBuf) -> OxiOptions {
  let mut compression = HashSet::new();
  compression.insert(9);

  let mut strategies = HashSet::new();
  for i in 0..4 {
    strategies.insert(i);
  }

  let mut filter = HashSet::new();
  for i in 0..6 {
    filter.insert(i);
  }

  let mut memory = HashSet::new();
  for i in 6..10 {
    memory.insert(i);
  }

  let verbosity: Option<u8> = match max_log_level() {
    LogLevelFilter::Error => None,
    LogLevelFilter::Info => Some(2),
    LogLevelFilter::Debug => Some(1),
    _ => None,
  };

  OxiOptions {
    backup: false,
    out_file: out_png,
    out_dir: None,
    stdout: false,
    fix_errors: true,
    pretend: false,
    recursive: false,
    clobber: true,
    create: false,
    force: false,
    preserve_attrs: false,
    verbosity: verbosity,
    filter: filter,
    interlace: None,
    compression: compression,
    memory: memory,
    strategies: strategies,
    window: 15u8,
    bit_depth_reduction: true,
    color_type_reduction: true,
    palette_reduction: true,
    idat_recoding: true,
    strip: Headers::All,
    use_heuristics: false,
    threads: 12,
  }
}
