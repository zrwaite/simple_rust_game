use crate::{Velocity, Position, MovementAnimation};
use specs_derive::Component;
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Character {
	pub velocity: Velocity,
	pub position: Position,
	pub animation: MovementAnimation,
}

impl Character {
}