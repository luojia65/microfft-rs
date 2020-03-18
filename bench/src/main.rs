#![no_std]
#![no_main]
#![allow(dead_code)]

use cortex_m::iprint;
use cortex_m_rt::entry;
use hal::{prelude::*, time::MonoTimer};
use num_complex::Complex32;
use panic_semihosting as _;

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

    let timer = MonoTimer::new(core.DWT, clocks);
    let cycles = bench::run(timer);

    let mut itm = core.ITM;
    iprint!(&mut itm.stim[0], "{}", cycles);

    panic!("bench done");
}

#[cfg(feature = "microfft-c")]
mod bench {
    use super::{n, timeit, Complex32, MonoTimer};
    use heapless::{consts::U4096, Vec};

    static mut X: Vec<Complex32, U4096> = Vec(heapless::i::Vec::new());

    pub fn run(timer: MonoTimer) -> u32 {
        let x = unsafe { &mut X };
        for i in 0..n::N {
            x.push(Complex32::new(i as f32, 0.)).unwrap();
        }

        timeit(timer, || n::CFFT(x))
    }
}

#[cfg(feature = "microfft-r")]
mod bench {
    use super::{n, timeit, MonoTimer};
    use heapless::{consts::U4096, Vec};

    static mut X: Vec<f32, U4096> = Vec(heapless::i::Vec::new());

    pub fn run(timer: MonoTimer) -> u32 {
        let x = unsafe { &mut X };
        for i in 0..n::N {
            x.push(i as f32).unwrap();
        }

        timeit(timer, || n::RFFT(x))
    }
}

#[cfg(feature = "fourier-c")]
mod bench {
    use super::{n, timeit, Complex32, MonoTimer};
    use fourier::Fft;
    use heapless::{consts::U4096, Vec};

    static mut X: Vec<Complex32, U4096> = Vec(heapless::i::Vec::new());

    pub fn run(timer: MonoTimer) -> u32 {
        let x = unsafe { &mut X };
        for i in 0..n::N {
            x.push(Complex32::new(i as f32, 0.)).unwrap();
        }

        timeit(timer, || n::Fourier.fft_in_place(x))
    }
}

fn timeit<F, R>(timer: MonoTimer, f: F) -> u32
where
    F: FnOnce() -> R,
{
    let instant = timer.now();
    let _ = f();
    instant.elapsed()
}

type FnCfft = fn(&mut [Complex32]) -> &mut [Complex32];
type FnRfft = fn(&mut [f32]) -> &mut [Complex32];

#[cfg(feature = "n-4")]
mod n {
    pub const N: usize = 4;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_4;
    pub const RFFT: super::FnRfft = microfft::real::rfft_4;

    #[fourier::static_fft(f32, 4)]
    pub struct Fourier;
}

#[cfg(feature = "n-8")]
mod n {
    pub const N: usize = 8;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_8;
    pub const RFFT: super::FnRfft = microfft::real::rfft_8;

    #[fourier::static_fft(f32, 8)]
    pub struct Fourier;
}

#[cfg(feature = "n-16")]
mod n {
    pub const N: usize = 16;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_16;
    pub const RFFT: super::FnRfft = microfft::real::rfft_16;

    #[fourier::static_fft(f32, 16)]
    pub struct Fourier;
}

#[cfg(feature = "n-32")]
mod n {
    pub const N: usize = 32;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_32;
    pub const RFFT: super::FnRfft = microfft::real::rfft_32;

    #[fourier::static_fft(f32, 32)]
    pub struct Fourier;
}

#[cfg(feature = "n-64")]
mod n {
    pub const N: usize = 64;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_64;
    pub const RFFT: super::FnRfft = microfft::real::rfft_64;

    #[fourier::static_fft(f32, 64)]
    pub struct Fourier;
}

#[cfg(feature = "n-128")]
mod n {
    pub const N: usize = 128;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_128;
    pub const RFFT: super::FnRfft = microfft::real::rfft_128;

    #[fourier::static_fft(f32, 128)]
    pub struct Fourier;
}

#[cfg(feature = "n-256")]
mod n {
    pub const N: usize = 256;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_256;
    pub const RFFT: super::FnRfft = microfft::real::rfft_256;

    #[fourier::static_fft(f32, 256)]
    pub struct Fourier;
}

#[cfg(feature = "n-512")]
mod n {
    pub const N: usize = 512;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_512;
    pub const RFFT: super::FnRfft = microfft::real::rfft_512;

    #[fourier::static_fft(f32, 512)]
    pub struct Fourier;
}

#[cfg(feature = "n-1024")]
mod n {
    pub const N: usize = 1024;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_1024;
    pub const RFFT: super::FnRfft = microfft::real::rfft_1024;

    #[fourier::static_fft(f32, 1024)]
    pub struct Fourier;
}

#[cfg(feature = "n-2048")]
mod n {
    pub const N: usize = 2048;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_2048;
    pub const RFFT: super::FnRfft = microfft::real::rfft_2048;

    #[fourier::static_fft(f32, 2048)]
    pub struct Fourier;
}

#[cfg(feature = "n-4096")]
mod n {
    pub const N: usize = 4096;
    pub const CFFT: super::FnCfft = microfft::complex::cfft_4096;
    pub const RFFT: super::FnRfft = microfft::real::rfft_4096;

    #[fourier::static_fft(f32, 4096)]
    pub struct Fourier;
}
