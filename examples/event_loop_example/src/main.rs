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
