use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window, EventPump};
use specs::{prelude::*, Builder};
use std::{thread, time::Duration};

mod colors;
mod components;
mod systems;
use components::{Position, Sprite};
mod tileset;

const INITIAL_WIDTH: u32 = 800;
const INITIAL_HEIGHT: u32 = 600;

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

fn main() -> Result<(), String> {
  let (canvas, mut events) = init()?;

  let mut world = World::new();
  world.register::<Position>();
  world.register::<Sprite>();

  world
    .create_entity()
    .with(Position { x: 4, y: 7 })
    .with(Sprite {
      id: "t_dirt".to_string(),
    })
    .build();
  world
    .create_entity()
    .with(Position { x: 3, y: 9 })
    .with(Sprite {
      id: "t_dirt".to_string(),
    })
    .build();

  let mut dispatcher = DispatcherBuilder::new()
    .with_thread_local(systems::RenderingSystem::new(canvas))
    .build();

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

    thread::sleep(Duration::from_millis(100));
  }
  Ok(())
}
