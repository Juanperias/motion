extern crate sdl2;
use record::collision::shape::Shape;
use record::collision::{self, Collision2d};
use record::forces::gravity::{gravity, EARTH_GRAVITY};
use record::forces::r#move::{move2d, Direction2d};
use record::formulas::elastic_collision;
use record::obj::obj_2d::{obj2d, Object2d};
use record::vec::vec_2d::vec2;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use std::collections;
use std::time::{Duration, Instant};

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
        vec2(75.0, 200.0),
        2.0,
        2.0,
        vec2(2.0, 231.0),
        vec2(2.0, -240.0),
        20.0,
        Shape::Circle,
    );
    let mut obj2 = obj2d(
        vec2(300.0, 200.0),
        2.0,
        60.0,
        vec2(0.0, 0.0),
        vec2(0.0, 0.0),
        20.0,
        Shape::Circle,
    );

    let mut event_pump = sdl_context.event_pump()?;
    let fps = 60.0;
    let dt = 1.0 / fps;
    let frame_duration = Duration::from_secs_f32(dt as f32);

    'main: loop {
        let frame_start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        //obj.apply(gravity(EARTH_GRAVITY, dt, dt));
        obj.apply(move2d(Direction2d::X, dt, 60.0));
        obj2.apply(move2d(Direction2d::X, dt, 60.0));
        let collision = Collision2d { obj1: obj, obj2 };

        if collision.collider() {
            let u = elastic_collision::calculate(obj.velocity, obj.mass, obj2.mass, obj2.velocity);

            let u2 = elastic_collision::calculate(obj2.velocity, obj2.mass, obj.mass, obj.velocity);
            obj2.velocity = u2;
            obj.velocity = u;
            obj.acceleration = u;
            obj2.acceleration = u2;
        }

        canvas.filled_circle(
            obj.vec.x as i16,
            obj.vec.y as i16,
            obj.radius as i16,
            Color::RGB(0, 0, 255),
        )?;

        canvas.filled_circle(
            obj2.vec.x as i16,
            obj.vec.y as i16,
            obj.radius as i16,
            Color::BLUE,
        )?;
        canvas.present();

        let frame_time = frame_start.elapsed();
        if frame_time < frame_duration {
            ::std::thread::sleep(frame_duration - frame_time);
        }
    }

    Ok(())
}
