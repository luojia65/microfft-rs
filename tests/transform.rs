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

fn micro_fft(input: &[Complex32]) -> Vec<Complex32> {
    let mut buf = input.to_vec();
    microfft::transform_1024(&mut buf);
    buf
}

fn approx_equal(a: Complex32, b: Complex32) -> bool {
    (a.re - b.re).abs() < 0.1 && (a.im - b.im).abs() < 0.1
}

#[test]
fn transform() {
    let input: Vec<_> = (0..1024)
        .map(|i| i as f32)
        .map(|f| Complex32::new(f, f))
        .collect();

    let result_rust = rust_fft(&input);
    let result_micro = micro_fft(&input);

    for (r, m) in result_rust.into_iter().zip(result_micro) {
        assert!(approx_equal(r, m));
    }
}
