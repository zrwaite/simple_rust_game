

use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (
        WriteStorage<'a, Position>, 
        ReadStorage<'a, Velocity>
    );

    fn run(&mut self, mut data: Self::SystemData) {      
        for (pos, vel) in (&mut data.0, &data.1).join() {
            pos.0 = pos.0.offset(vel.x, vel.y);
        }
    }
}

