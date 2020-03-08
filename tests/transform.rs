use num_complex::Complex32;
use rustfft::{algorithm::Radix4, FFT};

fn rust_fft(input: &[Complex32]) -> Vec<Complex32> {
    let len = input.len();
    let fft = Radix4::new(len, false);
    let mut input = input.to_vec();
    let mut output = vec![Complex32::default(); len];
    fft.process(&mut input, &mut output);
    output
}

fn approx_eq(a: Complex32, b: Complex32) -> bool {
    fn approx_f32(x: f32, y: f32) -> bool {
        let diff = (x - y).abs();
        let rel_diff = if x != 0. { (diff / x).abs() } else { diff };
        rel_diff < 0.02
    }

    approx_f32(a.re, b.re) && approx_f32(a.im, b.im)
}

fn assert_approx_eq(xa: &[Complex32], xb: &[Complex32]) {
    assert_eq!(xa.len(), xb.len());
    for (a, b) in xa.into_iter().zip(xb) {
        assert!(approx_eq(*a, *b));
    }
}

macro_rules! cfft_tests {
    ( $( $name:ident: $N:expr, )* ) => {
        $(
            #[test]
            fn $name() {
                let mut input: Vec<_> = (0..$N)
                    .map(|i| i as f32)
                    .map(|f| Complex32::new(f, f))
                    .collect();

                let expected = rust_fft(&input);
                let result = microfft::complex::$name(&mut input);

                assert_approx_eq(result, &expected);
            }
        )*
    };
}

cfft_tests! {
    cfft_2: 2,
    cfft_4: 4,
    cfft_8: 8,
    cfft_16: 16,
    cfft_32: 32,
    cfft_64: 64,
    cfft_128: 128,
    cfft_256: 256,
    cfft_512: 512,
    cfft_1024: 1024,
    cfft_2048: 2048,
    cfft_4096: 4096,
}

macro_rules! rfft_tests {
    ( $( $name:ident: ($N:expr, $cfft_name:ident), )* ) => {
        $(
            #[test]
            fn $name() {
                let mut input: Vec<_> = (5..($N+5)).map(|i| i as f32).collect();
                let mut input_c: Vec<_> = input.iter().map(|f| Complex32::new(*f, 0.)).collect();

                let expected = microfft::complex::$cfft_name(&mut input_c);
                let result = microfft::real::$name(&mut input);

                assert_approx_eq(result, &expected[..($N / 2)]);
            }
        )*
    };
}

rfft_tests! {
    rfft_2: (2, cfft_2),
    rfft_4: (4, cfft_4),
    rfft_8: (8, cfft_8),
    rfft_16: (16, cfft_16),
    rfft_32: (32, cfft_32),
    rfft_64: (64, cfft_64),
    rfft_128: (128, cfft_128),
    rfft_256: (256, cfft_256),
    rfft_512: (512, cfft_512),
    rfft_1024: (1024, cfft_1024),
    rfft_2048: (2048, cfft_2048),
    rfft_4096: (4096, cfft_4096),
}
