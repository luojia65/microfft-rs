//! Library for computing fast fourier transforms on embedded systems.
//!
//! microfft provides an in-place implementation of the Radix-2 FFT algorithm.
//! All computations are performed directly on the input buffer and require no
//! additional allocations. This makes microfft suitable for `no_std`
//! environments.
//!
//! In addition to the standard FFT implementation on [`Complex32`] values
//! ([`complex`]), an implementation working on real (`f32`) input values is
//! provided ([`real`]). An `N`-point RFFT internally computes an `N/2`-point
//! CFFT, making it roughly twice as fast a the complex variant.
//!
//! # Example
//!
//! ```
//! use std::f32::consts::PI;
//!
//! // generate 16 samples of a sine wave at frequency 3
//! let sample_count = 16;
//! let signal_freq = 3.;
//! let sample_interval = 1. / sample_count as f32;
//! let mut samples: Vec<_> = (0..sample_count)
//!     .map(|i| (2. * PI * signal_freq * sample_interval * i as f32).sin())
//!     .collect();
//!
//! // compute the RFFT of the samples
//! let spectrum = microfft::real::rfft_16(&mut samples);
//!
//! // the spectrum has a spike at index `signal_freq`
//! let amplitudes: Vec<_> = spectrum.iter().map(|c| c.norm() as u32).collect();
//! assert_eq!(&amplitudes, &[0, 0, 0, 8, 0, 0, 0, 0]);
//! ```
//!
//! [`complex`]: complex/index.html
//! [`real`]: real/index.html
//! [`Complex32`]: type.Complex32.html

#![no_std]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod complex;
pub mod real;

pub use num_complex::Complex32;

mod cfft;
mod rfft;
mod tables;

use static_assertions::assert_cfg;

assert_cfg!(
    any(
        feature = "maxn-4",
        feature = "maxn-8",
        feature = "maxn-16",
        feature = "maxn-32",
        feature = "maxn-64",
        feature = "maxn-128",
        feature = "maxn-256",
        feature = "maxn-512",
        feature = "maxn-1024",
        feature = "maxn-2048",
        feature = "maxn-4096",
    ),
    "One of the `maxn-*` features of this crate must be set."
);
