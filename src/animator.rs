

use specs::prelude::*;

use crate::components::*;
use crate::character::Character;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Character>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;
        for (anim, sprite, vel) in (&mut data.0, &mut data.1, &data.2).join() {
            if vel.x == 0 && vel.y ==0 {
                continue;
            }
            let frames = match sprite.direction {
                Left => &anim.left_frames,
                Right => &anim.right_frames,
                Up => &anim.up_frames,
                Down => &anim.down_frames,
            };
            anim.current_timer = (anim.current_timer + 1) % anim.max_timer;
            if anim.current_timer == 0 {
                anim.current_frame = (anim.current_frame + 1) % 4;
            }
            let frame_index = match anim.current_frame {
                0|2 => 1,
                1 => 2,
                3 => 0,
                _ => 1
            };
            *sprite = frames[frame_index].clone();
        }
    }
}

