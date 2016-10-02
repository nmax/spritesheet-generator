extern crate oxipng;

use self::oxipng::{Options as OxiOptions, optimize as oxi_optimize};
use self::oxipng::headers::Headers;

use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn optimize<P: AsRef<Path>>(img_path: P) {
  let out = Path::new(img_path.as_ref());
  let opts = oxi_options(PathBuf::from(out));

  match oxi_optimize(&out, &opts) {
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
    verbosity: Some(2),
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
