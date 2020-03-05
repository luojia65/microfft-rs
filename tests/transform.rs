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

#[test]
fn cfft() {
    let mut input: Vec<_> = (0..4096)
        .map(|i| i as f32)
        .map(|f| Complex32::new(f, f))
        .collect();

    let expected = rust_fft(&input);
    let result = microfft::complex::cfft_4096(&mut input);

    assert_approx_eq(result, &expected);
}

#[test]
fn rfft() {
    let mut input: Vec<_> = (0..4096).map(|i| i as f32).collect();
    let mut input_c: Vec<_> = input.iter().map(|f| Complex32::new(*f, 0.)).collect();

    let expected = microfft::complex::cfft_4096(&mut input_c);
    let result = microfft::real::rfft_4096(&mut input);

    assert_approx_eq(result, &expected[..2048]);
}
