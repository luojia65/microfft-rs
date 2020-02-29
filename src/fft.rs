use crate::tables;
use num_complex::Complex32;

pub(crate) trait Fft {
    const N: usize;
    const M: usize = Self::N / 2;

    const LOG2_N: usize;
    const BITREV_TABLE: &'static [u16] = tables::BITREV[Self::LOG2_N];
    const TWIDDLE_TABLE: &'static [Complex32] = tables::TWIDDLE[Self::LOG2_N];

    type Half: Fft;

    #[inline]
    fn transform(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), Self::N);

        Self::bit_reverse(x);
        Self::compute_butterflies(x);
    }

    #[inline]
    fn bit_reverse(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), Self::N);

        for i in 0..x.len() {
            let j = Self::BITREV_TABLE[i] as usize;
            x.swap(i, j);
        }
    }

    #[inline]
    fn compute_butterflies(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), Self::N);

        let m = Self::M;
        Self::Half::compute_butterflies(&mut x[..m]);
        Self::Half::compute_butterflies(&mut x[m..]);

        for k in 0..m {
            let f = x[k];
            let s = x[k + m];
            let twiddle = Self::TWIDDLE_TABLE[k];
            let prod = twiddle * s;
            x[k] = f + prod;
            x[k + m] = f - prod;
        }
    }
}

pub(crate) struct FftN1;

impl Fft for FftN1 {
    const N: usize = 1;
    const LOG2_N: usize = 0;

    type Half = Self;

    #[inline]
    fn compute_butterflies(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), Self::N);
    }
}

macro_rules! fft_impls {
    ( $( $I:expr => ($N:expr, $FftI:ident, $Half:ident), )* ) => {
        $(
            pub(crate) struct $FftI;

            impl Fft for $FftI {
                const N: usize = $N;
                const LOG2_N: usize = $I;

                type Half = $Half;
            }
        )*
    };
}

fft_impls! {
     1 => (2, FftN2, FftN1),
     2 => (4, FftN4, FftN2),
     3 => (8, FftN8, FftN4),
     4 => (16, FftN16, FftN8),
     5 => (32, FftN32, FftN16),
     6 => (64, FftN64, FftN32),
     7 => (128, FftN128, FftN64),
     8 => (256, FftN256, FftN128),
     9 => (512, FftN512, FftN256),
    10 => (1024, FftN1024, FftN512),
}
