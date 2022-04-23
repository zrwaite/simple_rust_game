use specs::prelude::*;
use specs_derive::Component;
use sdl2::rect::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

/// The current position of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

/// The current speed and direction of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    /// The specific spritesheet to render from
    pub spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    pub region: Rect,
    pub direction: Direction,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    // The current frame in the animation of the direction this entity is moving in
    pub current_frame: usize,
    pub current_timer: u32,
    pub max_timer: u32,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}
impl MovementAnimation {
    pub fn new(spritesheet: usize, top_left_frame: Rect) -> MovementAnimation {
        MovementAnimation {
            current_frame: 0,
            current_timer: 0,
            max_timer: 8,
            up_frames: MovementAnimation::get_animation_frame(spritesheet, top_left_frame, Direction::Up),
            down_frames: MovementAnimation::get_animation_frame(spritesheet, top_left_frame, Direction::Down),
            left_frames: MovementAnimation::get_animation_frame(spritesheet, top_left_frame, Direction::Left),
            right_frames: MovementAnimation::get_animation_frame(spritesheet, top_left_frame, Direction::Right),
        }
    }
    pub fn get_animation_frame(spritesheet: usize, top_left_frame: Rect, direction: Direction) -> Vec<Sprite> {
        let (frame_width, frame_height) = top_left_frame.size();
        let y_offset = top_left_frame.y() + frame_height as i32 * MovementAnimation::direction_spritesheet_row(direction);
    
        let mut frames = Vec::new();
        for i in 0..3 {
            frames.push(Sprite {
                spritesheet,
                region: Rect::new(
                    top_left_frame.x() + frame_width as i32 * i,
                    y_offset,
                    frame_width,
                    frame_height,
                ),
                direction: Direction::Down
            })
        }
    
        frames
    }
    fn direction_spritesheet_row( direction: Direction) -> i32 {
        use self::Direction::*;
        match direction {
            Up => 3,
            Down => 0,
            Left => 1,
            Right => 2,
        }
    }
}

#[derive(Component, Debug, Copy, Clone)]
#[storage(VecStorage)]
pub struct KeyTracker {
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool
}
impl KeyTracker {
    pub fn new () -> KeyTracker {
        KeyTracker {
            up: false,
            down: false,
            right: false,
            left: false
        }
    }
}