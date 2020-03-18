# Benchmarks

This code is used to run benchmarks on an embedded ARM Cortex-M4 system,
specifically the [STM32F3DISCOVERY][1] board.

It measures the number of CPU cycles required to compute both complex and real
FFTs of all sizes supported by microfft. As a point of comparison, the same
benchmarks are also performed against the [Fourier crate][2] which, at the time
of writing, seems to be the only other Rust FFT library with `no_std` support.

## Running

To run the benchmarks, make sure the `thumbv7em-none-eabihf` rustc target
and OpenOCD are installed and the board is connected. Then just execute
the `run.py` script.

`run.py` starts OpenOCD in the background. It then builds for every FFT-size
combination a benchmark binary, flashes it onto the board, and runs it.
The results are printed to stdout.

## Results

The following table lists the benchmark results (2020-03-08).

Measurements are in CPU cycles, so lower is better.

| FFT size | microfft (CFFT) | microfft (RFFT) | Fourier (CFFT) |
| -------: | --------------: | --------------: | -------------: |
|    **4** |              81 |              44 |            564 |
|    **8** |             213 |             145 |          1,462 |
|   **16** |             750 |             399 |          2,202 |
|   **32** |           2,321 |           1,274 |          4,173 |
|   **64** |           6,127 |           3,347 |         10,943 |
|  **128** |          15,395 |           8,286 |         20,904 |
|  **256** |          37,294 |          19,551 |         42,724 |
|  **512** |          89,326 |          46,743 |         97,380 |
| **1024** |         200,830 |         106,376 |          s/o\* |
| **2048** |         462,405 |         240,441 |          s/o\* |
| **4096** |       1,017,526 |         528,020 |          s/o\* |

\* FFT cannot be computed due to stack overflow.

[1]: https://www.st.com/en/evaluation-tools/stm32f3discovery.html
[2]: https://crates.io/crates/fourier
