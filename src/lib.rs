#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

mod arity;
#[cfg(feature = "build")]
pub mod build;
mod compute;
pub mod error;
pub mod fact;
pub mod horn;
mod internment;
mod ir;
pub use compute::compute;

type Error = error::TypefrogError;
type Result<T> = std::result::Result<T, Error>;
