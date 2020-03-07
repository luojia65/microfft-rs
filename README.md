# microfft

microfft is a library for computing fast fourier transforms that targets
embedded systems. It provides an in-place implementation of the Radix-2 FFT
algorithm. All computations are performed directly on the input buffer and
require no additional allocations. This makes microfft suitable for `no_std`
environments, like microcontrollers.

Speed is achieved mainly by maintaining pre-computed tables of the necessary
twiddle factors and bit-reversal indices. By replacing arithmetic operations
with simple memory lookups, we reduce a) the number of CPU cycles spent and
b) the overall complexity and size of the code, which in turn leads to less
pressure on the instruction cache. Unfortunately, those pre-computed tables
also claim a considerable amount of memory, which might be a deal-breaker for
some embedded projects (see [Limitations](#limitations)).

microfft also implements a specialized algorithm for FFTs on real (instead
of complex) values. Naively one would calculate a real FFT simply by converting
the input to complex values (leaving the imaginary part empty) and running a
CFFT. microfft's RFFT algorithm instead packs pairs of real values into
a single complex one each, then computes a CFFT of half the original input
size, followed by some recombination magic. This has the effect of roughly
halving the number of CPU cycles required, as can be seen in the
[benchmark results][1].

## Example

The following example demonstrates computing a 16-point RFFT on a set of
samples generated from a sine signal:

```rust
use std::f32::consts::PI;

// generate 16 samples of a sine wave at frequency 3
let sample_count = 16;
let signal_freq = 3.;
let sample_interval = 1. / sample_count as f32;
let mut samples: Vec<_> = (0..sample_count)
    .map(|i| (2. * PI * signal_freq * sample_interval * i as f32).sin())
    .collect();

// compute the RFFT of the samples
let spectrum = microfft::real::rfft_16(&mut samples);

// the spectrum has a spike at index `signal_freq`
let amplitudes: Vec<_> = spectrum.iter().map(|c| c.norm() as u32).collect();
assert_eq!(&amplitudes, &[0, 0, 0, 8, 0, 0, 0, 0]);
```

## Requirements

Requires Rust version **1.37.0** or newer.

## Limitations <a name="limitations"></a>

microfft has a few limitations, mostly due to its focus on speed, that might
make it unsuitable for some embedded projects. You should know about these
if you consider using this library:

### Memory Usage

The use of pre-computed tables for twiddle factors and bit reversal means that
microfft has considerable requirements on read-only memory. If your chip
doesn't have much flash to begin with, this can be an issue.

The amount of memory required for tables depends on the point-size of the FFT
that is computed:

| FFT size | table size [Bytes] |
| -------: | -----------------: |
|    **2** |                  4 |
|    **4** |                  8 |
|    **8** |                 20 |
|   **16** |                 48 |
|   **32** |                108 |
|   **64** |                232 |
|  **128** |                484 |
|  **256** |                992 |
|  **512** |              2,012 |
| **1024** |              4,056 |
| **2048** |              8,148 |
| **4096** |             16,336 |

These memory usage values apply to both CFFTs and RFFTs.

In addition, the code size also increases with FFT size.

### Supported FFT Sizes

microfft only supports FFT point-sizes that are powers of two, a
limitation of the Radix-2 algorithm. Additionally, the maximum supported size
is currently 4096, although this limit can easily be increased in the future
as necessary.

### `f64` Support

This library currently only supports single-precision floating-point inputs.
Similarly to the FFT size limit, this is a restriction that might be lifted
in the future, should the need arise.

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or
http://opensource.org/licenses/MIT).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in microfft by you, shall be licensed as above, without any
additional terms or conditions.

[1]: bench/README.md
