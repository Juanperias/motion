extern crate sdl2;

use record::forces::gravity::{gravity, EARTH_GRAVITY};
use record::obj::obj_2d::obj2d;
use record::vec::vec_2d::vec2;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Sdl2 + record", 600, 500)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut obj = obj2d(
        vec2(10.0, 200.0),
        2.0,
        2.0,
        vec2(10.0, 231.0),
        vec2(10.0, -240.0),
        20.0,
    );
    let mut event_pump = sdl_context.event_pump()?;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        obj.apply(gravity(EARTH_GRAVITY, 0.1));
        canvas.filled_circle(
            obj.vec.x as i16,
            obj.vec.y as i16,
            obj.radius as i16,
            Color::RGB(0, 0, 255),
        )?;

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
