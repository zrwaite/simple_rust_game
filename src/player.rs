use crate::controller;
use crate::sprite;
use sprite::{ Vector2, Sprite, Direction};
use controller::Controller;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};
// use sdl2::render::{Texture};



#[derive(Debug)]
pub struct Xy {
	
}

#[derive(Debug)]
pub struct Player {
	pub position: Point,
	pub sprite: Sprite,
	pub speed: Vector2,
	pub max_speed: i32,
	pub controller: Controller,
	pub direction: Direction,
	moving: bool
}

impl Player {
	pub fn new(position: Point, sprite: Sprite) -> Player{
		Player{
			position,
			sprite,
			max_speed: 3,
			speed: Vector2::new(0,0),
			controller: Controller::new(),
			direction: Direction::Down,
			moving: true
		}
	}
	pub fn update(&mut self) {
		self.controls();
		self.animation();
	}
	pub fn animation(&mut self) {
		self.sprite.region.set_y(self.sprite.start_region.y() + self.get_render_row()*self.sprite.region.height() as i32);
		self.sprite.region.set_x(self.sprite.start_region.x() + self.get_render_column()*self.sprite.region.width() as i32);
		self.sprite.animation(self.moving);
	}
	pub fn render(&self, canvas: &mut WindowCanvas, textures: &[Texture])-> Result<(), String> {
		let (width, height) = canvas.output_size()?;
		let screen_position = self.position + Point::new(width as i32/2, height as i32 / 2);
		let screen_rect = Rect::from_center(screen_position, self.sprite.size.x as u32, self.sprite.size.y as u32);
		canvas.copy(&textures[self.sprite.spritesheet], self.sprite.region, screen_rect)?;
		Ok(())
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
		self.moving = self.controller.left || self.controller.right || self.controller.up || self.controller.down;
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
	fn get_render_column(&self) -> i32 {
		if self.sprite.animation_frame==0 || self.sprite.animation_frame==2 {1} 
		else if self.sprite.animation_frame==1 {2} 
		else {0}
	}
}	