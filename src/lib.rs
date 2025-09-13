#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

mod arity;
mod compute;
pub mod fact;
pub mod horn;
mod internment;
mod ir;
pub use compute::compute;
