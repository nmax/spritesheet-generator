extern crate image;
extern crate handlebars;


use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum SpriteSheetError {
  Image(image::ImageError),
  TemplateRender(handlebars::TemplateRenderError),
  Io(io::Error),
}

impl fmt::Display for SpriteSheetError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      SpriteSheetError::Io(ref err) => write!(f, "IO error: {}", err),
      SpriteSheetError::TemplateRender(ref err) => {
        write!(f, "Template error: {}", err)
      }
      SpriteSheetError::Image(ref err) => write!(f, "Image error: {}", err),
    }
  }
}

impl error::Error for SpriteSheetError {
  fn description(&self) -> &str {
    match *self {
      SpriteSheetError::Io(ref err) => err.description(),
      SpriteSheetError::TemplateRender(ref err) => err.description(),
      SpriteSheetError::Image(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      SpriteSheetError::Io(ref err) => err.cause(),
      SpriteSheetError::TemplateRender(ref err) => err.cause(),
      SpriteSheetError::Image(ref err) => err.cause(),
    }
  }
}



impl From<image::ImageError> for SpriteSheetError {
  fn from(err: image::ImageError) -> SpriteSheetError {
    SpriteSheetError::Image(err)
  }
}

impl From<handlebars::TemplateRenderError> for SpriteSheetError {
  fn from(err: handlebars::TemplateRenderError) -> SpriteSheetError {
    SpriteSheetError::TemplateRender(err)
  }
}

impl From<io::Error> for SpriteSheetError {
  fn from(err: io::Error) -> SpriteSheetError {
    SpriteSheetError::Io(err)
  }
}
