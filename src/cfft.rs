use crate::tables;
use num_complex::Complex32;

pub(crate) trait CFft {
    type Half: CFft;

    const N: usize;
    const LOG2_N: usize;

    const M: usize = Self::N / 2;

    const BITREV_TABLE: &'static [u16] = tables::BITREV[Self::LOG2_N];
    const TWIDDLE_TABLE: &'static [Complex32] = tables::TWIDDLE[Self::LOG2_N];

    #[inline]
    fn transform(x: &mut [Complex32]) -> &mut [Complex32] {
        debug_assert_eq!(x.len(), Self::N);

        Self::bit_reverse(x);
        Self::compute_butterflies(x);
        x
    }

    #[inline]
    fn bit_reverse(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), Self::N);

        for i in 0..Self::N {
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

pub(crate) struct CFftN1;

impl CFft for CFftN1 {
    type Half = Self;

    const N: usize = 1;
    const LOG2_N: usize = 0;

    #[inline]
    fn compute_butterflies(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), Self::N);
    }
}

macro_rules! cfft_impls {
    ( $( $I:expr => ($N:expr, $CFftN:ident, $Half:ident), )* ) => {
        $(
            pub(crate) struct $CFftN;

            impl CFft for $CFftN {
                type Half = $Half;

                const N: usize = $N;
                const LOG2_N: usize = $I;
            }
        )*
    };
}

cfft_impls! {
     1 => (2, CFftN2, CFftN1),
     2 => (4, CFftN4, CFftN2),
     3 => (8, CFftN8, CFftN4),
     4 => (16, CFftN16, CFftN8),
     5 => (32, CFftN32, CFftN16),
     6 => (64, CFftN64, CFftN32),
     7 => (128, CFftN128, CFftN64),
     8 => (256, CFftN256, CFftN128),
     9 => (512, CFftN512, CFftN256),
    10 => (1024, CFftN1024, CFftN512),
}
