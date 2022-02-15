#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod zip_array;
#[cfg(feature = "alloc")]
pub use zip_array::*;

pub use futures_macros_lite::*;
