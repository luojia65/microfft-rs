use crate::{cfft::*, tables};
use core::{mem, slice};
use num_complex::Complex32;
use static_assertions::const_assert_eq;

pub(crate) trait RFft {
    type Fft: CFft;

    const N: usize = Self::Fft::N * 2;
    const LOG2_N: usize = Self::Fft::LOG2_N + 1;
    const TWIDDLE_TABLE: &'static [Complex32] = tables::TWIDDLE[Self::LOG2_N];

    #[inline]
    fn transform(x: &mut [f32]) -> &mut [Complex32] {
        debug_assert_eq!(x.len(), Self::N);

        let x = Self::pack_complex(x);

        Self::Fft::transform(x);
        Self::recombine(x);
        x
    }

    #[inline]
    fn pack_complex(x: &mut [f32]) -> &mut [Complex32] {
        const_assert_eq!(mem::size_of::<Complex32>(), mem::size_of::<f32>() * 2);
        const_assert_eq!(mem::align_of::<Complex32>(), mem::align_of::<f32>());
        assert_eq!(x.len(), Self::N);

        let data = x.as_ptr() as *mut Complex32;
        let len = Self::N / 2;

        // Drop the old mutable reference to `data` before creating a
        // new one to obey Rust's aliasing rules.
        #[allow(clippy::drop_ref)]
        drop(x);

        unsafe { slice::from_raw_parts_mut(data, len) }
    }

    #[inline]
    fn recombine(x: &mut [Complex32]) {
        let n = Self::Fft::N;
        debug_assert_eq!(x.len(), n);

        // DC
        let x0 = x[0];
        x[0] = Complex32::new(x0.re + x0.im, 0.);

        let m = n / 2 + 1;
        for k in 1..m {
            let xk = x[k];
            let xnk = x[n - k];

            let sum = (xk + xnk) / 2.;
            let diff = (xk - xnk) / 2.;

            let twiddle = Self::TWIDDLE_TABLE[k];
            x[k] = Complex32::new(
                sum.re + twiddle.re * sum.im + twiddle.im * diff.re,
                diff.im + twiddle.im * sum.im - twiddle.re * diff.re,
            );

            let twiddle = Self::TWIDDLE_TABLE[n - k];
            x[n - k] = Complex32::new(
                sum.re + twiddle.re * sum.im - twiddle.im * diff.re,
                -diff.im + twiddle.im * sum.im + twiddle.re * diff.re,
            );
        }
    }
}

macro_rules! rfft_impls {
    ( $( ($RFftN:ident, $CFftN:ident), )* ) => {
        $(
            pub(crate) struct $RFftN;

            impl RFft for $RFftN {
                type Fft = $CFftN;
            }
        )*
    };
}

rfft_impls! {
    (RFftN2, CFftN1),
    (RFftN4, CFftN2),
    (RFftN8, CFftN4),
    (RFftN16, CFftN8),
    (RFftN32, CFftN16),
    (RFftN64, CFftN32),
    (RFftN128, CFftN64),
    (RFftN256, CFftN128),
    (RFftN512, CFftN256),
    (RFftN1024, CFftN512),
}
