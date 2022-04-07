use sdl2::rect::{Point, Rect};
#[path="./controller.rs"] mod controller;
use controller::Controller;
// use sdl2::render::{Texture};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Xy {
	x: i32, y: i32
}

#[derive(Debug)]
pub struct Player {
	pub position: Point,
	pub sprite: Rect,
	pub start_sprite: Rect,
	pub speed: Xy,
	pub max_speed: i32,
	pub controller: Controller,
	pub direction: Direction
}

impl Player {
	pub fn new(position: Point, sprite: Rect) -> Player{
		Player{
			position: position,
			sprite: sprite,
			start_sprite: sprite,
			max_speed: 3,
			speed: Xy{x:0,y:0},
			controller: Controller::new(),
			direction: Direction::Down
		}
	}
	pub fn update(&mut self) {
		self.controls();
		self.render();
	}
	fn controls (&mut self) {
		if self.controller.left {
			self.direction = Direction::Left;
			self.speed.x = -self.max_speed;
		} else if self.controller.right {
			self.direction = Direction::Right;
			self.speed.x = self.max_speed;
		} else {self.speed.x = 0;}
		if self.controller.up {
			self.direction = Direction::Up;
			self.speed.y = -self.max_speed;
		} else if self.controller.down {
			self.speed.y = self.max_speed;
			self.direction = Direction::Down;
		} else {self.speed.y = 0;}
		self.position = self.position.offset(self.speed.x, self.speed.y);
	}
	fn render(&mut self) {
		self.sprite.set_y(self.start_sprite.y() + self.get_render_row()*self.sprite.height() as i32);
	}
	fn get_render_row(&self) -> i32 {
		use self::Direction::*;
		match self.direction {
			Down => 0,
			Left => 1,
			Right => 2,
			Up => 3,
		}
	}
}	