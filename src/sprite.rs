use specs::prelude::*;
use specs_derive::Component;
use sdl2::rect::{Rect};


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Vector2 {
    pub x: i32, 
	pub y: i32
}
impl Vector2 {
	pub fn new(x: i32, y:i32) -> Vector2 {
		Vector2{x,y}
	}
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
   	pub start_region: Rect,
	pub region: Rect
}
impl Sprite {
	pub fn new(spritesheet: usize, region: Rect) -> Sprite{
		Sprite{
			spritesheet,
			region,
			start_region: region,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct MovementAnimation {
    // The current frame in the animation of the direction this entity is moving in
    current_frame: usize,
    up_frames: Vec<Sprite>,
    down_frames: Vec<Sprite>,
    left_frames: Vec<Sprite>,
    right_frames: Vec<Sprite>,
}