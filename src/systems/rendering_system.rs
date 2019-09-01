use crate::colors::*;
use rand::{thread_rng, Rng};
use sdl2::{
  event::Event,
  image::{InitFlag, LoadSurface, LoadTexture},
  keyboard::Keycode,
  pixels::{Color, PixelFormatEnum},
  rect::Rect,
  render::{BlendMode, Canvas, Texture, TextureAccess, TextureCreator, TextureQuery},
  surface::Surface,
  video::Window,
  EventPump,
};
use serde::{Deserialize, Serialize};
use specs::{prelude::*, Builder, Component, Join, ReadStorage, Storage, System, VecStorage};
use specs_derive::Component;
use std::{
  path::Path,
  rc::Rc,
  sync::{Arc, Mutex},
  thread,
  time::{Duration, Instant},
};

use crate::components::{Position, Sprite};

use crate::tileset;

const SPRITE_H: u32 = 32;
const SPRITE_W: u32 = 32;

pub struct RenderingSystem<'a> {
  pub canvas: Rc<Canvas<Window>>,
  pub tileset: tileset::Tileset<'a>,
}

impl<'a> RenderingSystem<'a> {
  pub fn new(canvas: Canvas<Window>) -> RenderingSystem<'a> {
    RenderingSystem {
      canvas: Rc::new(canvas),
      tileset: tileset::load_tileset(),
    }
  }

  fn render(&mut self, position: ReadStorage<Position>, sprite: ReadStorage<Sprite>) {
    let canvas = Rc::get_mut(&mut self.canvas).unwrap();
    canvas.set_draw_color(GRAY);
    canvas.clear();
    for (position, sprite) in (&position, &sprite).join() {
      println!("pos {}, {}, sprite {}", position.x, position.y, sprite.id);
      let tile_id = self.tileset.tiles.get(&sprite.id).unwrap().fg.unwrap();
      let row = tile_id % (self.tileset.width / self.tileset.tile_width) as i32;
      let column = tile_id / (self.tileset.width / self.tileset.tile_width) as i32;
      println!(
        "tileset.width {}, tileset.height {}",
        self.tileset.width, self.tileset.height
      );
      println!(
        "tileset.width / tile_width {}, tileset.height / tile_height {}",
        self.tileset.width / self.tileset.tile_width,
        self.tileset.height / self.tileset.tile_height
      );
      println!("tile_id {}, row {}, column {}", tile_id, row, column);
      let src_rect = Rect::new(
        (row * self.tileset.tile_width as i32),
        (column * self.tileset.tile_height as i32),
        self.tileset.tile_width,
        self.tileset.tile_height,
      );
      let dest_rect = Rect::new(
        position.x * self.tileset.tile_width as i32,
        position.y * self.tileset.tile_height as i32,
        self.tileset.tile_width,
        self.tileset.tile_height,
      );
      println!("src_rect {:?}, ", src_rect);
      println!("dest_rect {:?}, ", dest_rect);
      let raw_sprite: Surface = Self::raw_sprite(&self.tileset.spritesheet, src_rect).unwrap();
      let texture_creator = canvas.texture_creator();
      let sprite_tex: Texture = texture_creator.create_texture_from_surface(&raw_sprite).unwrap();
      canvas.copy(&sprite_tex, None, Some(dest_rect)).unwrap();
    }
    canvas.present();
  }

  fn raw_sprite(spritesheet: &Surface, sprite_rect: Rect) -> Result<Surface<'a>, String> {
    let mut tile: Surface = Self::create_tile(sprite_rect)?;
    spritesheet.blit(sprite_rect, &mut tile, None)?;
    Ok(tile)
  }

  fn create_tile(size_rect: Rect) -> Result<Surface<'a>, String> {
    let tile: Surface = Surface::new(size_rect.width(), size_rect.height(), PixelFormatEnum::RGB888)?;
    Ok(tile)
  }
}

impl<'a> System<'a> for RenderingSystem<'_> {
  type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

  fn run(&mut self, (position, sprite): Self::SystemData) {
    self.render(position, sprite);
  }
}
