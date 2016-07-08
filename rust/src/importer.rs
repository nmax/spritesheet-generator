use sprite::Sprite;
use errors::SpriteSheetError;
use std::path::Path;

pub fn load_files(files: Vec<&str>) -> Result<Vec<Sprite>, SpriteSheetError> {
  files.iter()
    .map(|f| Sprite::new(&Path::new(f)))
    .collect()
}
