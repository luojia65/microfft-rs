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
|    **2** |              25 |              13 |            728 |
|    **4** |              71 |              32 |            798 |
|    **8** |             189 |             107 |          1,611 |
|   **16** |             654 |             357 |          2,353 |
|   **32** |           2,279 |           1,127 |          4,411 |
|   **64** |           6,034 |           3,319 |         11,515 |
|  **128** |          15,261 |           8,189 |         21,307 |
|  **256** |          36,716 |          19,438 |         44,499 |
|  **512** |          87,900 |          46,128 |        103,878 |
| **1024** |         200,214 |         104,768 |        200,818 |

[1]: https://www.st.com/en/evaluation-tools/stm32f3discovery.html
[2]: https://crates.io/crates/fourier
