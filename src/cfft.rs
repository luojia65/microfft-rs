use crate::tables;
use num_complex::Complex32;

pub(crate) trait CFft {
    type Next: CFft;

    const N: usize;
    const LOG2_N: usize;

    #[cfg(feature = "bitrev-tables")]
    const BITREV_TABLE: &'static [u16] = tables::BITREV[Self::LOG2_N];

    #[inline]
    fn transform(x: &mut [Complex32]) -> &mut [Complex32] {
        debug_assert_eq!(x.len(), Self::N);

        Self::bit_reverse_reorder(x);
        Self::compute_butterflies(x);
        x
    }

    #[cfg(feature = "bitrev-tables")]
    #[inline]
    fn bit_reverse_reorder(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), Self::N);

        for i in 0..Self::N {
            let j = Self::BITREV_TABLE[i] as usize;
            x.swap(i, j);
        }
    }

    #[cfg(not(feature = "bitrev-tables"))]
    #[inline]
    fn bit_reverse_reorder(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), Self::N);

        let shift = core::mem::size_of::<usize>() * 8 - Self::LOG2_N;
        for i in 0..Self::N {
            let rev = i.reverse_bits();
            let j = rev >> shift;
            if j > i {
                x.swap(i, j);
            }
        }
    }

    #[inline]
    fn compute_butterflies(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), Self::N);

        let q = Self::N / 4;
        let q2 = 2 * q;
        let q3 = 3 * q;
        let i = Complex32::i();

        for chunk in x.chunks_exact_mut(q) {
            Self::Next::compute_butterflies(chunk);
        }

        // [k = 0] twiddle factors: `1 + 0i`
        let (x0, x1, x2, x3) = (x[0], x[q], x[q2], x[q3]);
        x[0] = x0 + x1 + x2 + x3;
        x[q] = x0 - x1 - (x2 * i) + (x3 * i);
        x[q2] = x0 + x1 - x2 - x3;
        x[q3] = x0 - x1 + (x2 * i) - (x3 * i);

        for k in 1..q {
            let (x0, x1, x2, x3) = (x[k], x[k + q], x[k + q2], x[k + q3]);
            let y1 = x2 * Self::twiddle(k);
            let y2 = x1 * Self::twiddle(2 * k);
            let y3 = x3 * Self::twiddle(3 * k);
            x[k] = x0 + y1 + y2 + y3;
            x[k + q] = x0 - (y1 * i) - y2 + (y3 * i);
            x[k + q2] = x0 - y1 + y2 - y3;
            x[k + q3] = x0 + (y1 * i) - y2 - (y3 * i);
        }
    }

    #[inline]
    fn twiddle(k: usize) -> Complex32 {
        let q = Self::N / 4;
        let q2 = 2 * q;
        let q3 = 3 * q;

        let table_len = tables::SINE.len();
        let table_stride = (table_len + 1) / q;
        let s = (k % q) * table_stride;

        let (re, im) = if k == 0 {
            (1., 0.)
        } else if k < q {
            (-tables::SINE[table_len - s], tables::SINE[s - 1])
        } else if k == q {
            (0., -1.)
        } else if k < q2 {
            (tables::SINE[s - 1], tables::SINE[table_len - s])
        } else if k == q2 {
            (-1., 0.)
        } else if k < q3 {
            (tables::SINE[table_len - s], -tables::SINE[s - 1])
        } else if k == q3 {
            (0., 1.)
        } else {
            (-tables::SINE[s - 1], -tables::SINE[table_len - s])
        };

        Complex32::new(re, im)
    }
}

pub(crate) struct CFftN1;

impl CFft for CFftN1 {
    type Next = Self;

    const N: usize = 1;
    const LOG2_N: usize = 0;

    #[inline]
    fn bit_reverse_reorder(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), 1);
    }

    #[inline]
    fn compute_butterflies(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), 1);
    }
}

pub(crate) struct CFftN2;

impl CFft for CFftN2 {
    type Next = CFftN1;

    const N: usize = 2;
    const LOG2_N: usize = 1;

    #[inline]
    fn compute_butterflies(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), 2);

        let (x0, x1) = (x[0], x[1]);
        x[0] = x0 + x1;
        x[1] = x0 - x1;
    }
}

pub(crate) struct CFftN4;

impl CFft for CFftN4 {
    type Next = CFftN1;

    const N: usize = 4;
    const LOG2_N: usize = 2;

    #[inline]
    fn compute_butterflies(x: &mut [Complex32]) {
        debug_assert_eq!(x.len(), 4);

        let i = Complex32::i();

        let (x0, x1, x2, x3) = (x[0], x[1], x[2], x[3]);
        x[0] = x0 + x1 + x2 + x3;
        x[1] = x0 - x1 - (x2 * i) + (x3 * i);
        x[2] = x0 + x1 - x2 - x3;
        x[3] = x0 - x1 + (x2 * i) - (x3 * i);
    }
}

macro_rules! cfft_impls {
    ( $( $I:expr => ($N:expr, $CFftN:ident, $Next:ident), )* ) => {
        $(
            #[allow(dead_code)]
            pub(crate) struct $CFftN;

            impl CFft for $CFftN {
                type Next = $Next;

                const N: usize = $N;
                const LOG2_N: usize = $I;
            }
        )*
    };
}

cfft_impls! {
     3 => (8, CFftN8, CFftN2),
     4 => (16, CFftN16, CFftN4),
     5 => (32, CFftN32, CFftN8),
     6 => (64, CFftN64, CFftN16),
     7 => (128, CFftN128, CFftN32),
     8 => (256, CFftN256, CFftN64),
     9 => (512, CFftN512, CFftN128),
    10 => (1024, CFftN1024, CFftN256),
    11 => (2048, CFftN2048, CFftN512),
    12 => (4096, CFftN4096, CFftN1024),
}
