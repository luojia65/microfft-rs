# Changelog

All notable changes to this project will be documented in this file.

## 0.2.0 (2020-03-08)

### Changed

- Bitrev tables are not used anymore by default, instead the bit-reversed
  indices are computed directly at runtime. This significantly reduces the
  memory usage of microfft. On architectures that provide a dedicated
  bit-reversal instruction (like `RBIT` on ARMv7), speed is also increased.
  The `bitrev-tables` feature can be enabled to still opt into using bitrev
  tables.


## 0.1.2 (2020-03-07)

### Changed

- Store pre-computed sine values instead of full twiddles, reducing the size
  of the twiddle tables to one fourth the prior size.

## 0.1.1 (2020-03-05)

### Added

- Support for FFT sizes 2048 and 4096.


## 0.1.0 (2020-03-04)

### Added

- Support for complex and real FFTs up to size 1024.
