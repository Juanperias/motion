#![no_std]
#![allow(clippy::module_name_repetitions)]

//! # Motion, Bare Metal physics engine
//! **Motion** is a bare metal physics engine which is created to be easy to use.
//! ## Get started!
//! you can first add the package with
//! ```bash
//! cargo add motion
//! ```
//! With this you already have record installed in your project, you can start with a simple event loop
//!```rust
//!use std::{thread, time::Duration};
//!
//!use motion::event_loop::EventLoopBuilder;
//!
// The definition of this function depends on the context in which motion is used
//!fn sleep(duration: Duration) {
//!    thread::sleep(duration);
//!}
//!
//!fn main() {
//!    let el = EventLoopBuilder::new().fps(1).build();
//!
//!    el.start(|_config| println!("Hello! in the event loop"), sleep);
//!}
//! https://github.com/Juanperias/motion/blob/main/examples/event_loop_example/src/main.rs
//!```
//! More examples in <https://github.com/Juanperias/motion/tree/main/examples>
//!

#[cfg(feature = "vec")]
pub mod vec;

#[cfg(feature = "obj")]
pub mod obj;

#[cfg(feature = "formulas")]
pub mod formulas;

#[cfg(feature = "forces")]
pub mod forces;

#[cfg(feature = "event_loop")]
pub mod event_loop;

#[cfg(feature = "collision")]
pub mod collision;
