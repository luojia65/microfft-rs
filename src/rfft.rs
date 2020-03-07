use crate::{cfft::*, tables};
use core::{mem, slice};
use num_complex::Complex32;
use static_assertions::const_assert_eq;

pub(crate) trait RFft {
    type CFft: CFft;

    const N: usize = Self::CFft::N * 2;
    const LOG2_N: usize = Self::CFft::LOG2_N + 1;
    const SINE_TABLE: &'static [f32] = tables::SINE[Self::LOG2_N - 2];

    #[inline]
    fn transform(x: &mut [f32]) -> &mut [Complex32] {
        debug_assert_eq!(x.len(), Self::N);

        let x = Self::pack_complex(x);

        Self::CFft::transform(x);
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
        let n = Self::CFft::N;
        debug_assert_eq!(x.len(), n);

        // DC
        let x0 = x[0];
        x[0] = Complex32::new(x0.re + x0.im, 0.);

        let m = n / 2;
        for k in 1..m {
            let twiddle_re = Self::SINE_TABLE[m - k - 1] * -1.;
            let twiddle_im = Self::SINE_TABLE[k - 1];

            let (x_k, x_nk) = (x[k], x[n - k]);
            let sum = (x_k + x_nk) / 2.;
            let diff = (x_k - x_nk) / 2.;

            x[k] = Complex32::new(
                sum.re + twiddle_re * sum.im + twiddle_im * diff.re,
                diff.im + twiddle_im * sum.im - twiddle_re * diff.re,
            );
            x[n - k] = Complex32::new(
                sum.re - twiddle_re * sum.im - twiddle_im * diff.re,
                -diff.im + twiddle_im * sum.im - twiddle_re * diff.re,
            );
        }

        x[m] *= Complex32::new(0., -1.);
    }
}

pub(crate) struct RFftN2;

impl RFft for RFftN2 {
    type CFft = CFftN1;

    #[inline]
    fn recombine(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), 1);

        // DC
        let x0 = x[0];
        x[0] = Complex32::new(x0.re + x0.im, 0.);
    }
}

macro_rules! rfft_impls {
    ( $( ($RFftN:ident, $CFftN:ident), )* ) => {
        $(
            pub(crate) struct $RFftN;

            impl RFft for $RFftN {
                type CFft = $CFftN;
            }
        )*
    };
}

rfft_impls! {
    (RFftN4, CFftN2),
    (RFftN8, CFftN4),
    (RFftN16, CFftN8),
    (RFftN32, CFftN16),
    (RFftN64, CFftN32),
    (RFftN128, CFftN64),
    (RFftN256, CFftN128),
    (RFftN512, CFftN256),
    (RFftN1024, CFftN512),
    (RFftN2048, CFftN1024),
    (RFftN4096, CFftN2048),
}
