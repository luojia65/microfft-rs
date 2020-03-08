//! FFT on complex inputs (CFFT)

use crate::cfft::*;
use num_complex::Complex32;

/// Perform an in-place 2-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_2};
///
/// let mut input = [Complex32::default(); 2];
/// let result = cfft_2(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `2`.
#[inline]
pub fn cfft_2(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 2);
    CFftN2::transform(input)
}

/// Perform an in-place 4-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_4};
///
/// let mut input = [Complex32::default(); 4];
/// let result = cfft_4(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `4`.
#[inline]
pub fn cfft_4(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 4);
    CFftN4::transform(input)
}

/// Perform an in-place 8-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_8};
///
/// let mut input = [Complex32::default(); 8];
/// let result = cfft_8(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `8`.
#[cfg(any(
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
))]
#[inline]
pub fn cfft_8(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 8);
    CFftN8::transform(input)
}

/// Perform an in-place 16-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_16};
///
/// let mut input = [Complex32::default(); 16];
/// let result = cfft_16(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `16`.
#[cfg(any(
    feature = "maxn-16",
    feature = "maxn-32",
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn cfft_16(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 16);
    CFftN16::transform(input)
}

/// Perform an in-place 32-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_32};
///
/// let mut input = [Complex32::default(); 32];
/// let result = cfft_32(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `32`.
#[cfg(any(
    feature = "maxn-32",
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn cfft_32(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 32);
    CFftN32::transform(input)
}

/// Perform an in-place 64-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_64};
///
/// let mut input = [Complex32::default(); 64];
/// let result = cfft_64(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `64`.
#[cfg(any(
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn cfft_64(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 64);
    CFftN64::transform(input)
}

/// Perform an in-place 128-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_128};
///
/// let mut input = [Complex32::default(); 128];
/// let result = cfft_128(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `128`.
#[cfg(any(
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn cfft_128(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 128);
    CFftN128::transform(input)
}

/// Perform an in-place 256-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_256};
///
/// let mut input = [Complex32::default(); 256];
/// let result = cfft_256(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `256`.
#[cfg(any(
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn cfft_256(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 256);
    CFftN256::transform(input)
}

/// Perform an in-place 512-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_512};
///
/// let mut input = [Complex32::default(); 512];
/// let result = cfft_512(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `512`.
#[cfg(any(
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn cfft_512(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 512);
    CFftN512::transform(input)
}

/// Perform an in-place 1024-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_1024};
///
/// let mut input = [Complex32::default(); 1024];
/// let result = cfft_1024(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `1024`.
#[cfg(any(feature = "maxn-1024", feature = "maxn-2048", feature = "maxn-4096"))]
#[inline]
pub fn cfft_1024(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 1024);
    CFftN1024::transform(input)
}

/// Perform an in-place 2048-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_2048};
///
/// let mut input = [Complex32::default(); 2048];
/// let result = cfft_2048(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `2048`.
#[cfg(any(feature = "maxn-2048", feature = "maxn-4096"))]
#[inline]
pub fn cfft_2048(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 2048);
    CFftN2048::transform(input)
}

/// Perform an in-place 4096-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_4096};
///
/// let mut input = [Complex32::default(); 4096];
/// let result = cfft_4096(&mut input);
/// ```
///
/// # Panics
///
/// Panics if `input` has a length other than `4096`.
#[cfg(any(feature = "maxn-4096"))]
#[inline]
pub fn cfft_4096(input: &mut [Complex32]) -> &mut [Complex32] {
    assert_eq!(input.len(), 4096);
    CFftN4096::transform(input)
}
