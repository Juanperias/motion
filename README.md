<img src="https://i.ibb.co/dDJLb0X/motion-Banner.png">

# Motion🍃

Motion is a **bare metal physics engine** with which you can make simulations easily and quickly, also in rust.

## Get started ✨

let's start by making a simple event loop

```rust
use std::{thread, time::Duration};

use motion::event_loop::EventLoopBuilder;

// The definition of this function depends on the context in which motion is used
fn sleep(duration: Duration) {
    thread::sleep(duration);
}

fn main() {
    let el = EventLoopBuilder::new().fps(1).build();

    el.start(|_config| println!("Hello! in the event loop"), sleep);
}
```

now we are going to do something more complex by creating an object

```rust
    let obj = Object2dBuilder::new()
        .position(vec2(2.0, 2.0))
        .density(2.0)
        .mass(3.0)
        .velocity(vec2(4.0, 4.0))
        .acceleration(vec2(3.0, 3.0))
        .radius(2.0)
        .shape(Shape::Circle)
        .build();
```

## Why rust 🦀

Rust is a fast and efficient programming language, which makes it perfect for motion, plus it is very flexible allowing motion to be used everywhere.
