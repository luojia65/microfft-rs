//! Fast fourier transforms without heap-allocation.
//!
//! microfft provides an in-place implementation of the radix-2 DIT FFT
//! algorithm. All computations are performed directly on the input buffer
//! and require no additional allocations. This makes microfft suitable for
//! `no_std` environments.

#![no_std]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod complex;
pub mod real;

pub use num_complex::Complex32;

mod cfft;
mod rfft;
mod tables;
