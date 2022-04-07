// use sdl2::rect::{Point, Rect};
// use sdl2::render::{Texture};

#[derive(Debug)]
pub struct Controller {
	pub left: bool,
	pub right: bool,
	pub up: bool,
	pub down: bool
}

impl Controller {
	pub fn new() -> Controller{
		Controller{
			left: false,
			right: false,
			up: false,
			down: false,
		}
	}
}	