# Benchmarks

This code is used to run benchmarks on an embedded ARM Cortex-M4 system,
specifically the [STM32F3DISCOVERY][1] board.

It measures the number of CPU cycles required to compute both complex and
real FFTs of all sizes currently supported by microfft. As a point of
comparison, the same benchmarks are also performed against the
[Fourier crate][2] which, at the time of writing, seems to be the only other
Rust FFT library with `no_std` support.

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

The following table lists the benchmark results (2020-03-02).

Measurements are in CPU cycles, so lower is better.

| FFT size | microfft (CFFT) | microfft (RFFT) | Fourier (CFFT) |
| -------: | --------------: | --------------: | -------------: |
|    **2** |              40 |              11 |            727 |
|    **4** |              98 |              59 |            801 |
|    **8** |             249 |             144 |           1602 |
|   **16** |             778 |             425 |           2338 |
|   **32** |            2331 |            1495 |           4374 |
|   **64** |            7120 |            3741 |          11480 |
|  **128** |           17506 |            9933 |          21240 |
|  **256** |           41471 |           23182 |          44370 |
|  **512** |           95600 |           53610 |         103366 |
| **1024** |          224224 |          117375 |         199793 |

[1]: https://www.st.com/en/evaluation-tools/stm32f3discovery.html
[2]: https://crates.io/crates/fourier
