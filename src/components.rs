use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
  pub id: String,
}
