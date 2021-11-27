// #![deny(warnings)]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(feature = "simd", feature(core_intrinsics))]
#![cfg_attr(feature = "bench", feature(test))]

#[macro_use]
extern crate cfg_if;

pub(crate) mod internals;

mod types;
pub use types::*;

#[cfg(feature = "cffi")]
mod c_ffi;
#[cfg(feature = "cffi")]
pub use c_ffi::*;

#[cfg(test)]
mod tests {
    pub trait RoundTo<T> {
        fn round_to(&self, n: usize) -> T;
    }

    impl RoundTo<f32> for f32 {
        fn round_to(&self, n: usize) -> f32 {
            let factor = 10.0_f32.powi(n as i32);
            (self * factor).round() / factor
        }
    }
}
