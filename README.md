<img src="https://i.ibb.co/dDJLb0X/motion-Banner.png">

# Motion🍃

Motion is a **bare metal physics engine** with which you can make simulations easily and quickly, also in rust.

## Get started ✨

let's start by making a simple event loop

```rust
use std::{thread, time::Duration};

use motion::event_loop::{EventLoop, EventLoopConfig};

// The definition of this function depends on the context in which motion is used
fn sleep(duration: Duration) {
    thread::sleep(duration);
}

fn main() {
    let el = EventLoop::new(EventLoopConfig { fps: 1 });

    el.start(|_config| println!("Hello! in the event loop"), sleep);
}
```

now we are going to do something more complex by creating an object

```rust
    let mut obj = obj2d(
        vec2(75.0, 200.0),
        2.0,
        2.0,
        vec2(2.0, 231.0),
        vec2(2.0, -240.0),
        20.0,
        Shape::Circle,
    );
```

## Why rust 🦀

Rust is a fast and efficient programming language, which makes it perfect for motion, plus it is very flexible allowing motion to be used everywhere.
