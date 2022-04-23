

use specs::prelude::*;

use crate::components::*;

// use super::MovementCommand;

const PLAYER_MOVEMENT_SPEED: i32 = 3;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, KeyTracker>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Sprite>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let presses = &*data.0;
        for ( _, vel, sprite) in (&data.1, &mut data.2, &mut data.3).join() {
            if presses.down {
                vel.y = PLAYER_MOVEMENT_SPEED;
                sprite.direction = Direction::Down;
            } else if presses.up {
                vel.y = - PLAYER_MOVEMENT_SPEED;
                sprite.direction = Direction::Up;
            } else {
                vel.y = 0;
            }
            if presses.left {
                vel.x = - PLAYER_MOVEMENT_SPEED;
                sprite.direction = Direction::Left;
            } else if presses.right {
                vel.x = PLAYER_MOVEMENT_SPEED;
                sprite.direction = Direction::Right;
            } else {
                vel.x = 0;
            }
        }   
    }
}
