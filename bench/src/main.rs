#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use fourier::Fft;
use hal::{prelude::*, time::MonoTimer};
use heapless::{consts::U1024, Vec};
use num_complex::Complex32;
use panic_semihosting as _;

static mut XC: Vec<Complex32, U1024> = Vec(heapless::i::Vec::new());
static mut XF: Vec<f32, U1024> = Vec(heapless::i::Vec::new());

#[entry]
fn main() -> ! {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = hal::stm32::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let rcc = device.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .hclk(72.mhz())
        .pclk1(36.mhz())
        .pclk2(72.mhz())
        .freeze(&mut flash.acr);

    let xc = unsafe { &mut XC };
    (0..1024)
        .map(|i| Complex32::new(i as f32, 0.))
        .for_each(|c| xc.push(c).unwrap());

    let xf = unsafe { &mut XF };
    (0..1024)
        .map(|i| i as f32)
        .for_each(|f| xf.push(f).unwrap());

    let timer = MonoTimer::new(core.DWT, clocks);

    bench_2(timer, xc, xf);
    bench_4(timer, xc, xf);
    bench_8(timer, xc, xf);
    bench_16(timer, xc, xf);
    bench_32(timer, xc, xf);
    bench_64(timer, xc, xf);
    bench_128(timer, xc, xf);
    bench_256(timer, xc, xf);
    bench_512(timer, xc, xf);
    bench_1024(timer, xc, xf);

    panic!("bench done");
}

fn timeit<F, R>(timer: MonoTimer, f: F) -> u32
where
    F: FnOnce() -> R,
{
    let instant = timer.now();
    let _ = f();
    instant.elapsed()
}

macro_rules! benches {
    ( $( $N:expr => ($bench_N:ident, $cfft_N:ident, $rfft_N:ident, $FourierN:ident), )* ) => {
        $(
            #[fourier::static_fft(f32, $N)]
            struct $FourierN;

            fn $bench_N(timer: MonoTimer, xc: &mut [Complex32], xf: &mut [f32]) {
                hprintln!("FFT {}pt:", $N).unwrap();

                let xc = &mut xc[..$N];
                let xf = &mut xf[..$N];

                let cycles = timeit(timer, || microfft::complex::$cfft_N(xc));
                hprintln!("- microfft (c): {}", cycles).unwrap();

                let cycles = timeit(timer, || microfft::real::$rfft_N(xf));
                hprintln!("- microfft (r): {}", cycles).unwrap();

                let cycles = timeit(timer, || $FourierN.fft_in_place(xc));
                hprintln!("- fourier:      {}", cycles).unwrap();
            }
        )*
    };
}

benches! {
    2 => (bench_2, cfft_2, rfft_2, Fourier2),
    4 => (bench_4, cfft_4, rfft_4, Fourier4),
    8 => (bench_8, cfft_8, rfft_8, Fourier8),
    16 => (bench_16, cfft_16, rfft_16, Fourier16),
    32 => (bench_32, cfft_32, rfft_32, Fourier32),
    64 => (bench_64, cfft_64, rfft_64, Fourier64),
    128 => (bench_128, cfft_128, rfft_128, Fourier128),
    256 => (bench_256, cfft_256, rfft_256, Fourier256),
    512 => (bench_512, cfft_512, rfft_512, Fourier512),
    1024 => (bench_1024, cfft_1024, rfft_1024, Fourier1024),
}
