#![no_std]

/// Welcome to the record documentation!

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
