use amethyst::{
  core::timing::Time,
  ecs::prelude::{Read, System, Write},
};

const FPS: f32 = 10.0;

pub struct XPerSecSystem;

use crate::game::{ElapsedTime, Turns};

impl<'s> System<'s> for XPerSecSystem {
  type SystemData = (Write<'s, Turns>, Write<'s, ElapsedTime>, Read<'s, Time>);

  fn run(&mut self, (mut turns, mut elapsed_time, time): Self::SystemData) {
    elapsed_time.x_per_sec = elapsed_time.x_per_sec + time.delta_seconds();
    if elapsed_time.x_per_sec >= 1.0 / FPS {
      elapsed_time.x_per_sec = 0.0;
      let mut current_turn = turns.x_per_sec;
      current_turn = current_turn + 1;
      turns.x_per_sec = current_turn;
    }
  }
}
