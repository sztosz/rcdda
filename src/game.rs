use amethyst::{
  assets::Loader,
  ecs::prelude::Entity,
  prelude::*,
  ui::{Anchor, TtfFormat, UiText, UiTransform},
};

#[derive(Default)]
pub struct Game;

impl SimpleState for Game {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    initializer_turn_display(world);
  }
}

#[derive(Default)]
pub struct Turns {
  pub as_fast_as_possible_turn: i128,
  pub x_per_sec: i128,
}

#[derive(Default)]
pub struct ElapsedTime {
  pub display_system: f32,
  pub x_per_sec: f32,
}

pub struct TurnsDisplay {
  pub as_fast_as_possible_turn: Entity,
  pub x_per_sec: Entity,
}

fn initializer_turn_display(world: &mut World) {
  let font =
    world
      .read_resource::<Loader>()
      .load("font/square.ttf", TtfFormat, (), &world.read_resource());
  let as_fast_as_possible_transform = UiTransform::new(
    "as_fast_as_possible".to_string(),
    Anchor::TopLeft,
    Anchor::TopLeft,
    0.0,
    0.0,
    1.0,
    1200.0,
    50.0,
  );

  let as_fast_as_possible_turn = world
    .create_entity()
    .with(as_fast_as_possible_transform)
    .with(UiText::new(
      font.clone(),
      "TURNS".to_string(),
      [1.0, 1.0, 1.0, 1.0],
      12.0,
    ))
    .build();
  let x_per_sec = UiTransform::new(
    "x_per_sec".to_string(),
    Anchor::BottomLeft,
    Anchor::BottomLeft,
    0.0,
    0.0,
    1.0,
    1200.0,
    50.0,
  );

  let x_per_sec = world
    .create_entity()
    .with(x_per_sec)
    .with(UiText::new(
      font.clone(),
      "FRAMES".to_string(),
      [1.0, 1.0, 1.0, 1.0],
      12.0,
    ))
    .build();

  world.add_resource(TurnsDisplay {
    as_fast_as_possible_turn,
    x_per_sec,
  });
}
