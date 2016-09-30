extern crate image;
use self::image::{DynamicImage, GenericImage, imageops};

use sprite::{Sprite, PlacedSprite};
use size::Size;
use bounding_rect::BoundingRect;

#[derive(Debug)]
pub enum PlacementStrategy {
  Packed,
  StackedVertical,
  StackedHorizontal,
}

impl PlacementStrategy {
  pub fn place_sprites(&self,
                       sprites: Vec<Sprite>)
                       -> (DynamicImage, Vec<PlacedSprite>) {
    let placed_sprites = match *self {
      PlacementStrategy::Packed => pack(sprites),
      PlacementStrategy::StackedVertical => stack_vertical(sprites),
      PlacementStrategy::StackedHorizontal => stack_horizontal(sprites),
    };

    let (width, height) = find_canvas_constraints(&placed_sprites);
    assert!(width > 0);
    assert!(height > 0);
    println!("Canvas size: {}x{}", width, height);
    let mut canvas = DynamicImage::new_rgba8(width, height);

    for sprite in &placed_sprites {
      render_sprite(sprite, &mut canvas);
    }

    (canvas, placed_sprites)
  }
}

fn render_sprite(psprite: &PlacedSprite, buffer: &mut DynamicImage) {
  let mut sub_image = imageops::crop(buffer,
                                     psprite.position.x,
                                     psprite.position.y,
                                     psprite.position.width,
                                     psprite.position.height);
  sub_image.copy_from(&psprite.sprite.buffer, 0, 0);
}

// TODO: Das sollte sich allgemeiner implementieren lassen.
fn find_canvas_constraints(placed_sprites: &Vec<PlacedSprite>) -> Size {
  placed_sprites.into_iter()
    .fold((0, 0), |size, sprite| {
      let (mut x_end, mut y_end) = size;
      if sprite.position.x + sprite.position.width > x_end {
        x_end = sprite.position.x + sprite.position.width;
      }

      if sprite.position.y + sprite.position.height > y_end {
        y_end = sprite.position.y + sprite.position.height;
      }

      (x_end, y_end)
    })
}

fn pack(mut sprites: Vec<Sprite>) -> Vec<PlacedSprite> {
  sprites.sort();
  let max_width = sprites[0].dimensions().0;
  let mut x_offset = 0;
  let mut y_offset = 0;
  let mut row_height = 0;

  sprites.into_iter()
    .enumerate()
    .map(|(n, sprite)| {
      let size = sprite.dimensions();
      if n == 0 {
        let bounds = BoundingRect::new(0, 0, size.0, size.1);
        y_offset = bounds.height;
        PlacedSprite::new(sprite, bounds)
      } else {

        // Append right
        if x_offset + size.0 < max_width {
          if size.1 > row_height {
            row_height = size.1;
          }

          let bounds = BoundingRect::new(x_offset, y_offset, size.0, size.1);
          x_offset = x_offset + size.0;
          PlacedSprite::new(sprite, bounds)
        } else {
          // begin new row
          x_offset = 0;

          // move down the length of the previous lines tallest sprite
          y_offset = y_offset + row_height;

          // reset the line height to the current sprites 'height
          row_height = size.1;

          let bounds = BoundingRect::new(x_offset, y_offset, size.0, size.1);
          x_offset = x_offset + size.0;
          PlacedSprite::new(sprite, bounds)
        }
      }
    })
    .collect()
}

fn stack_horizontal(sprites: Vec<Sprite>) -> Vec<PlacedSprite> {
  let mut offset: u32 = 0;
  sprites.into_iter()
    .map(|sprite| {
      let size = sprite.dimensions();
      let bounds = BoundingRect::new(offset, 0, size.0, size.1);
      offset += sprite.dimensions().0 + 1;
      PlacedSprite::new(sprite, bounds)
    })
    .collect()
}

fn stack_vertical(sprites: Vec<Sprite>) -> Vec<PlacedSprite> {
  let mut offset: u32 = 0;
  sprites.into_iter()
    .map(|sprite| {
      let size = sprite.dimensions();
      let bounds = BoundingRect::new(0, offset, size.0, size.1);
      offset += sprite.dimensions().1 + 1;
      PlacedSprite::new(sprite, bounds)
    })
    .collect()
}
