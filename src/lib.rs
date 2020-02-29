//! Fast fourier transforms without heap-allocation.
//!
//! microfft provides and in-place implementation of the radix-2 DIT FFT
//! algorithm. All computations are performed directly on the input buffer
//! and require no additional allocations. This makes microfft suitable for
//! `no_std` environments.

#![no_std]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

mod fft;
mod tables;

pub use num_complex::Complex32;

use crate::fft::*;

/// Perform an in-place 2-point FFT.
///
/// # Example
///
/// ```
/// use microfft::Complex32;
///
/// let mut x = [Complex32::default(); 2];
/// microfft::transform_2(&mut x);
/// ```
///
/// # Panics
///
/// Panics if `x` has a length other than `2`.
#[inline]
pub fn transform_2(x: &mut [Complex32]) {
    assert_eq!(x.len(), 2);
    FftN2::transform(x);
}

/// Perform an in-place 4-point FFT.
///
/// # Example
///
/// ```
/// use microfft::Complex32;
///
/// let mut x = [Complex32::default(); 4];
/// microfft::transform_4(&mut x);
/// ```
///
/// # Panics
///
/// Panics if `x` has a length other than `4`.
#[inline]
pub fn transform_4(x: &mut [Complex32]) {
    assert_eq!(x.len(), 4);
    FftN4::transform(x);
}

/// Perform an in-place 8-point FFT.
///
/// # Example
///
/// ```
/// use microfft::Complex32;
///
/// let mut x = [Complex32::default(); 8];
/// microfft::transform_8(&mut x);
/// ```
///
/// # Panics
///
/// Panics if `x` has a length other than `8`.
#[inline]
pub fn transform_8(x: &mut [Complex32]) {
    assert_eq!(x.len(), 8);
    FftN8::transform(x);
}

/// Perform an in-place 16-point FFT.
///
/// # Example
///
/// ```
/// use microfft::Complex32;
///
/// let mut x = [Complex32::default(); 16];
/// microfft::transform_16(&mut x);
/// ```
///
/// # Panics
///
/// Panics if `x` has a length other than `16`.
#[inline]
pub fn transform_16(x: &mut [Complex32]) {
    assert_eq!(x.len(), 16);
    FftN16::transform(x);
}

/// Perform an in-place 32-point FFT.
///
/// # Example
///
/// ```
/// use microfft::Complex32;
///
/// let mut x = [Complex32::default(); 32];
/// microfft::transform_32(&mut x);
/// ```
///
/// # Panics
///
/// Panics if `x` has a length other than `32`.
#[inline]
pub fn transform_32(x: &mut [Complex32]) {
    assert_eq!(x.len(), 32);
    FftN32::transform(x);
}

/// Perform an in-place 64-point FFT.
///
/// # Example
///
/// ```
/// use microfft::Complex32;
///
/// let mut x = [Complex32::default(); 64];
/// microfft::transform_64(&mut x);
/// ```
///
/// # Panics
///
/// Panics if `x` has a length other than `64`.
#[inline]
pub fn transform_64(x: &mut [Complex32]) {
    assert_eq!(x.len(), 64);
    FftN64::transform(x);
}

/// Perform an in-place 128-point FFT.
///
/// # Example
///
/// ```
/// use microfft::Complex32;
///
/// let mut x = [Complex32::default(); 128];
/// microfft::transform_128(&mut x);
/// ```
///
/// # Panics
///
/// Panics if `x` has a length other than `128`.
#[inline]
pub fn transform_128(x: &mut [Complex32]) {
    assert_eq!(x.len(), 128);
    FftN128::transform(x);
}

/// Perform an in-place 256-point FFT.
///
/// # Example
///
/// ```
/// use microfft::Complex32;
///
/// let mut x = [Complex32::default(); 256];
/// microfft::transform_256(&mut x);
/// ```
///
/// # Panics
///
/// Panics if `x` has a length other than `256`.
#[inline]
pub fn transform_256(x: &mut [Complex32]) {
    assert_eq!(x.len(), 256);
    FftN256::transform(x);
}

/// Perform an in-place 512-point FFT.
///
/// # Example
///
/// ```
/// use microfft::Complex32;
///
/// let mut x = [Complex32::default(); 512];
/// microfft::transform_512(&mut x);
/// ```
///
/// # Panics
///
/// Panics if `x` has a length other than `512`.
#[inline]
pub fn transform_512(x: &mut [Complex32]) {
    assert_eq!(x.len(), 512);
    FftN512::transform(x);
}

/// Perform an in-place 1024-point FFT.
///
/// # Example
///
/// ```
/// use microfft::Complex32;
///
/// let mut x = [Complex32::default(); 1024];
/// microfft::transform_1024(&mut x);
/// ```
///
/// # Panics
///
/// Panics if `x` has a length other than `1024`.
#[inline]
pub fn transform_1024(x: &mut [Complex32]) {
    assert_eq!(x.len(), 1024);
    FftN1024::transform(x);
}
