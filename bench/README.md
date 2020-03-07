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

The following table lists the benchmark results (2020-03-07).

Measurements are in CPU cycles, so lower is better.

| FFT size | microfft (CFFT) | microfft (RFFT) | Fourier (CFFT) |
| -------: | --------------: | --------------: | -------------: |
|    **2** |              25 |              13 |            722 |
|    **4** |              70 |              34 |            801 |
|    **8** |             185 |             106 |          1,609 |
|   **16** |             656 |             353 |          2,356 |
|   **32** |           1,929 |           1,117 |          4,406 |
|   **64** |           6,271 |           2,978 |         11,515 |
|  **128** |          15,644 |           8,408 |         21,309 |
|  **256** |          38,076 |          19,878 |         44,501 |
|  **512** |          90,363 |          47,451 |        103,872 |
| **1024** |         213,349 |         107,376 |        200,815 |

[1]: https://www.st.com/en/evaluation-tools/stm32f3discovery.html
[2]: https://crates.io/crates/fourier
