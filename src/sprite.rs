use image::{ imageops::FilterType, DynamicImage };
use piston_window::{ Texture, TextureSettings, G2dTextureContext, G2dTexture };

use crate::{ PIECE_HEIGHT, PIECE_WIDTH };

pub struct SpriteHandler<'a> {
  pub sprites: Vec<G2dTexture>,
  ctx: &'a mut G2dTextureContext,
  sheet: DynamicImage,
}

impl <'a>SpriteHandler<'a> {
  pub fn new(sheet: &str, ctx: &'a mut G2dTextureContext) -> Self {


    return SpriteHandler {
      sheet: image::open(sheet).unwrap(),
      sprites: Vec::new(),
      ctx
    }
  }

  pub fn load(&mut self) {
    let (sprite_width, sprite_height) = (self.sheet.width() / 6, self.sheet.height() / 2);

    for y in 0 .. 2 {
      for x in 0 .. 6 {
        let image = self.sheet.crop(
          (x * sprite_width) as u32,
          (y * sprite_height) as u32,
          sprite_width as u32,
          sprite_height as u32
        ).resize(PIECE_WIDTH as u32, PIECE_HEIGHT as u32, FilterType::Lanczos3);

        self.sprites.push(Texture::from_image(
            self.ctx,
            &image.into_rgba8(),
            &TextureSettings::new()).unwrap()
          );
      }
    }

  }
}