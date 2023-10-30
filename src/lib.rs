//! # Library for parsing numbers from scientific notation

#![no_std]

extern crate alloc;
extern crate core;

mod bid128;
mod number;
mod recognizer;
#[cfg(test)]
mod tests;

pub use bid128::{bid128_from_string, Bid128};
pub use number::{number_from_string, Number};
pub use recognizer::Rounding;
