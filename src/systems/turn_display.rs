use amethyst::{
  core::timing::Time,
  ecs::prelude::{Read, ReadExpect, System, Write, WriteStorage},
  ui::UiText,
};

use crate::game::{ElapsedTime, Turns, TurnsDisplay};

const FPS: f32 = 1.0;

pub struct TurnDisplaySystem;

impl<'s> System<'s> for TurnDisplaySystem {
  type SystemData = (
    Read<'s, Turns>,
    ReadExpect<'s, TurnsDisplay>,
    Write<'s, ElapsedTime>,
    WriteStorage<'s, UiText>,
    Read<'s, Time>,
  );

  fn run(&mut self, (turns, turns_display, mut elapsed_time, mut ui_text, time): Self::SystemData) {
    elapsed_time.display_system = elapsed_time.display_system + time.delta_seconds();
    if elapsed_time.display_system >= 1.0 / FPS {
      elapsed_time.display_system = 0.0;
      if let Some(text) = ui_text.get_mut(turns_display.as_fast_as_possible_turn) {
        text.text = format!("TURNS : {}", turns.as_fast_as_possible_turn)
      }
      if let Some(text) = ui_text.get_mut(turns_display.x_per_sec) {
        text.text = format!("FRAMES: {}", turns.x_per_sec)
      }
    }
  }
}
