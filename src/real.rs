//! FFT on real inputs (RFFT)

use crate::rfft::*;
use num_complex::Complex32;

/// Perform an in-place 2-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_2;
///
/// let mut input = [0.; 2];
/// let result = rfft_2(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `2`.
#[inline]
pub fn rfft_2(input: &mut [f32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 2);
    RFftN2::transform(input)
}

/// Perform an in-place 4-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_4;
///
/// let mut input = [0.; 4];
/// let result = rfft_4(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `4`.
#[inline]
pub fn rfft_4(input: &mut [f32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 4);
    RFftN4::transform(input)
}

/// Perform an in-place 8-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_8;
///
/// let mut input = [0.; 8];
/// let result = rfft_8(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `8`.
#[inline]
pub fn rfft_8(input: &mut [f32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 8);
    RFftN8::transform(input)
}

/// Perform an in-place 16-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_16;
///
/// let mut input = [0.; 16];
/// let result = rfft_16(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `16`.
#[inline]
pub fn rfft_16(input: &mut [f32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 16);
    RFftN16::transform(input)
}

/// Perform an in-place 32-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_32;
///
/// let mut input = [0.; 32];
/// let result = rfft_32(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `32`.
#[inline]
pub fn rfft_32(input: &mut [f32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 32);
    RFftN32::transform(input)
}

/// Perform an in-place 64-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_64;
///
/// let mut input = [0.; 64];
/// let result = rfft_64(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `64`.
#[inline]
pub fn rfft_64(input: &mut [f32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 64);
    RFftN64::transform(input)
}

/// Perform an in-place 128-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_128;
///
/// let mut input = [0.; 128];
/// let result = rfft_128(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `128`.
#[inline]
pub fn rfft_128(input: &mut [f32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 128);
    RFftN128::transform(input)
}

/// Perform an in-place 256-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_256;
///
/// let mut input = [0.; 256];
/// let result = rfft_256(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `256`.
#[inline]
pub fn rfft_256(input: &mut [f32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 256);
    RFftN256::transform(input)
}

/// Perform an in-place 512-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_512;
///
/// let mut input = [0.; 512];
/// let result = rfft_512(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `512`.
#[inline]
pub fn rfft_512(input: &mut [f32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 512);
    RFftN512::transform(input)
}

/// Perform an in-place 1024-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_1024;
///
/// let mut input = [0.; 1024];
/// let result = rfft_1024(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `1024`.
#[inline]
pub fn rfft_1024(input: &mut [f32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 1024);
    RFftN1024::transform(input)
}
