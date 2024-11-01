use std::{f32::consts::PI, thread, time::Duration};

use minifb::{Window, WindowOptions};
use motion::{
    collision::{shape::Shape, Collision2d},
    event_loop::EventLoopBuilder,
    forces::r#move::{move2d, Direction2d},
    formulas::elastic_collision,
    obj::obj_2d::Object2dBuilder,
    vec::vec_2d::vec2,
};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

const WIDTH: usize = 800;
const HEIGHT: usize = 500;

fn sleep(time: Duration) {
    thread::sleep(time);
}

/*
*this is an example of elastic collisions using motion for more information see their wikipedia page: https://en.wikipedia.org/wiki/Elastic_collision
*/

fn main() {
    let mut window = Window::new(
        "Motion + raqote",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let mut obj1 = Object2dBuilder::new()
        .position(vec2(10.0, 100.0))
        .radius(20.0)
        .mass(1.0)
        .density(2.0)
        .acceleration(vec2(2000.0, 10.0))
        .shape(Shape::Circle)
        .velocity(vec2(2000.0, 10.0))
        .build();

    let mut obj2 = Object2dBuilder::new()
        .position(vec2(300.0, 100.0))
        .radius(20.0)
        .mass(1.0)
        .density(2.0)
        .acceleration(vec2(0.0, 0.0))
        .shape(Shape::Circle)
        .velocity(vec2(0.0, 0.0))
        .build();

    let el = EventLoopBuilder::new().fps(60).build();

    el.start_mut(
        move |config| {
            dt.clear(SolidSource::from_unpremultiplied_argb(
                0xff, 0xff, 0xff, 0xff,
            ));
            let mut pb = PathBuilder::new();
            obj1.apply(&move2d(
                Direction2d::X,
                config.delta_time,
                config.delta_time,
            ));
            obj2.apply(&move2d(
                Direction2d::X,
                config.delta_time,
                config.delta_time,
            ));
            let collide = Collision2d::new(obj1, obj2);

            if collide.collider() {
                let u = elastic_collision::calculate(
                    obj1.velocity,
                    obj1.mass,
                    obj2.mass,
                    obj2.velocity,
                );
                let u2 = elastic_collision::calculate(
                    obj2.velocity,
                    obj2.mass,
                    obj1.mass,
                    obj1.velocity,
                );

                obj1.velocity = u;
                obj1.acceleration = u;

                obj2.velocity = u2;
                obj2.acceleration = u2;
            }

            pb.arc(obj1.vec.x, obj1.vec.y, obj1.radius, 0.0, 2.0 * PI);
            let path = pb.finish();
            dt.fill(
                &path,
                &Source::Solid(SolidSource::from_unpremultiplied_argb(
                    0xff, 0x00, 0x00, 0x00,
                )),
                &DrawOptions::new(),
            );

            let mut pb2 = PathBuilder::new();

            pb2.arc(obj2.vec.x, obj2.vec.y, obj2.radius, 0.0, 2.0 * PI);
            let path2 = pb2.finish();

            dt.fill(
                &path2,
                &Source::Solid(SolidSource::from_unpremultiplied_argb(
                    0xff, 0x00, 0x00, 0x00,
                )),
                &DrawOptions::new(),
            );

            window
                .update_with_buffer(dt.get_data(), size.0, size.1)
                .unwrap();
        },
        sleep,
    );
}
