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
use specs::{prelude::*, Builder, Component, Join, ReadStorage, System, VecStorage};
use specs_derive::Component;
use std::{
  path::Path,
  rc::Rc,
  sync::{Arc, Mutex},
  thread,
  time::{Duration, Instant},
};

mod colors;
mod error;
mod systems;
use error::GameError;
use systems::RenderingSystem;
mod components;
use components::{Position, Sprite};
mod tileset;

const INITIAL_WIDTH: u32 = 800;
const INITIAL_HEIGHT: u32 = 600;

const SPRITE_H: u32 = 32;
const SPRITE_W: u32 = 32;

const SPRITE_COLS: u32 = 16;
const SPRITE_ROWS: u32 = 325;

const SPRITE_SHEET: &'static str = "assets/tiles/tiles.png";

fn raw_sprite<'a>(spritesheet: &Surface, sprite_rect: Rect) -> Result<Surface<'a>, String> {
  let mut tile: Surface = create_tile(sprite_rect)?;
  spritesheet.blit(sprite_rect, &mut tile, None)?;
  Ok(tile)
}

fn create_tile<'a>(size_rect: Rect) -> Result<Surface<'a>, String> {
  let tile: Surface = Surface::new(size_rect.width(), size_rect.height(), PixelFormatEnum::RGB888)?;
  Ok(tile)
}

fn init() -> Result<(Canvas<Window>, EventPump), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let window = match video_subsystem
    .window("demo", INITIAL_WIDTH, INITIAL_HEIGHT)
    .resizable()
    .position_centered()
    .opengl()
    .build()
  {
    Ok(window) => Ok(window),
    Err(_error) => Err("Window initialization eror!".to_string()),
  }?;
  let canvas = match window.into_canvas().build() {
    Ok(canvas) => Ok(canvas),
    Err(_error) => Err("Window initialization eror!".to_string()),
  }?;

  let events = sdl_context.event_pump()?;
  Ok((canvas, events))
}

fn main() -> Result<(), GameError> {
  let (canvas, mut events) = init()?;

  let mut world = World::new();
  world.register::<Position>();
  world.register::<Sprite>();

  world
    .create_entity()
    .with(Position { x: 4, y: 7 })
    .with(Sprite { id: "10mm".to_string() })
    .build();
  world
    .create_entity()
    .with(Position { x: 3, y: 9 })
    .with(Sprite { id: "10mm".to_string() })
    .build();

  let mut dispatcher = DispatcherBuilder::new()
    .with_thread_local(RenderingSystem::new(canvas))
    .build();

  // let mut rng = thread_rng();

  // sprite_surface.set_color_key(true, TRANSPARENT).unwrap();

  // let texture_creator = canvas.texture_creator();
  // let mut spritesheet: Texture = texture_creator.create_texture_from_surface(&sprite_surface)?;
  // spritesheet.set_blend_mode(BlendMode::Add);

  // let tex_query: TextureQuery = spritesheet.query();

  // println!("tex_query.width == {}", tex_query.width);
  // println!(" SPRITE_W * SPRITE_COLS == {}", SPRITE_W * SPRITE_COLS);
  // println!("tex_query.height == {}", tex_query.height);
  // println!("SPRITE_H * SPRITE_ROWS == {}", SPRITE_H * SPRITE_ROWS);
  // assert!(tex_query.width == SPRITE_W * SPRITE_COLS);
  // assert!(tex_query.height == SPRITE_H * SPRITE_ROWS);

  'running: loop {
    for event in events.poll_iter() {
      match event {
        Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }
    dispatcher.dispatch(&mut world);
    world.maintain();

    // canvas.set_draw_color(GRAY);
    // canvas.clear();
    // for i in 0..(INITIAL_WIDTH / SPRITE_W) + 1 {
    //   for j in 0..(INITIAL_WIDTH / SPRITE_H) + 1 {
    //     let x = rng.gen_range(0, SPRITE_ROWS);
    //     let y = rng.gen_range(0, SPRITE_COLS);

    //     let dest_rect = Rect::new(
    //       (i * SPRITE_W) as i32,
    //       (j * SPRITE_H) as i32,
    //       SPRITE_W,
    //       SPRITE_H,
    //     );
    //     let src_rect = Rect::new(
    //       (x * SPRITE_W) as i32,
    //       (y * SPRITE_H) as i32,
    //       SPRITE_W,
    //       SPRITE_H,
    //     );

    //     let raw_sprite: Surface = raw_sprite(&sprite_surface, src_rect).unwrap();
    //     let sprite_tex: Texture = texture_creator
    //       .create_texture_from_surface(&raw_sprite)
    //       .unwrap();
    //     canvas.copy(&sprite_tex, None, Some(dest_rect)).unwrap();
    //     canvas.present();
    //   }
    // }
    // .unwrap().set(buffer_tex);
    // renderer.set_draw_color(BLUE);
    // renderer.clear();

    // canvas.set_draw_color(GRAY);
    // canvas.clear();
    // canvas.present();

    // let size = canvas.window().size();
    // println!("SIZE {} x {}", size.0, size.1);

    thread::sleep(Duration::from_millis(100));
  }
  Ok(())
}
