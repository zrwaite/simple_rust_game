mod components;
mod physics;
mod animator;
mod keyboard;
mod renderer;
mod character;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};

use specs::{WorldExt, Builder, SystemData};
use specs::prelude::{DispatcherBuilder, World};

use std::time::Duration;

use crate::components::{Position, Velocity, KeyboardControlled, KeyTracker, MovementAnimation};
use crate::character::Character;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(physics::Physics, "Physics", &["Keyboard"])
        .with(animator::Animator, "Animator", &["Keyboard"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    // Initialize resource
    let mut presses = KeyTracker::new();
    world.insert(presses);

    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
    ];
    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 26, 36);
    let player = Character{
        animation: MovementAnimation::new(player_spritesheet, player_top_left_frame),
        position: Position(Point::new(0, 0)),
        velocity: Velocity {x: 0, y: 0},
    };

    world.create_entity()
        .with(KeyboardControlled)
        .with(player.position)
        .with(player.velocity)
        .with(player.animation.right_frames[0].clone())
        .with(player.animation)
        .build();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    presses.left = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    presses.right = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    presses.up = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    presses.down = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    presses.left = false;
                }   
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    presses.right = false;
                }   
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    presses.up = false;
                }   
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    presses.down = false;
                },
                _ => {}
            }
        }

        *world.write_resource() = presses;

        // Update
        i = (i + 1) % 255;
        dispatcher.dispatch(&mut world);
        world.maintain();

        // Render
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), &textures, world.system_data())?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}