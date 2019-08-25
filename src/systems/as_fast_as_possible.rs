use amethyst::ecs::prelude::{ReadExpect, System, Write};

use crate::game::{Turns, TurnsDisplay};

pub struct AsFastAsPossibleSystem;

impl<'s> System<'s> for AsFastAsPossibleSystem {
  type SystemData = (Write<'s, Turns>, ReadExpect<'s, TurnsDisplay>);

  fn run(&mut self, (mut turns, _x): Self::SystemData) {
    let mut current_turn = turns.as_fast_as_possible_turn;
    current_turn = current_turn + 1;
    turns.as_fast_as_possible_turn = current_turn;
  }
}
