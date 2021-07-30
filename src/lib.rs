// #![deny(warnings)]
#![cfg_attr(feature = "simd", feature(portable_simd))]

#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate lazy_static;

pub(crate) mod internals;

mod types;
pub use types::*;

#[cfg(feature = "cffi")]
mod c_ffi;
#[cfg(feature = "cffi")]
pub use c_ffi::*;
