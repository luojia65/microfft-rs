# Benchmarks

This code is used to run benchmarks on an embedded ARM Cortex-M4 system,
specifically the [STM32F3DISCOVERY][1] board.

It measures the number of CPU cycles required to compute both complex and real
FFTs for up to 1024 input values. As a point of comparison, the same benchmarks
are also performed against the [Fourier crate][2] which, at the time of writing,
seems to be the only other Rust FFT library with `no_std` support.

## Running

To run the benchmarks, make sure the `thumbv7em-none-eabihf` rustc target
is installed and OpenOCD is running and connected to the board. Then just
execute:

```
$ cargo run --release
```

This builds the benchmark binary, flashes it onto the board, and runs it.
The results are printed to the OpenOCD session via semihosting.

## Results

The following table lists the benchmark results (2020-03-08).

Measurements are in CPU cycles, so lower is better.

| FFT size | microfft (CFFT) | microfft (RFFT) | Fourier (CFFT) |
| -------: | --------------: | --------------: | -------------: |
|    **2** |              25 |              13 |            722 |
|    **4** |              70 |              34 |            801 |
|    **8** |             185 |             106 |          1,609 |
|   **16** |             671 |             353 |          2,356 |
|   **32** |           2,281 |           1,135 |          4,406 |
|   **64** |           6,134 |           3,358 |         11,511 |
|  **128** |          15,126 |           8,258 |         21,309 |
|  **256** |          36,838 |          19,377 |         44,504 |
|  **512** |          88,021 |          46,122 |        103,872 |
| **1024** |         201,392 |         104,489 |        200,814 |

[1]: https://www.st.com/en/evaluation-tools/stm32f3discovery.html
[2]: https://crates.io/crates/fourier
