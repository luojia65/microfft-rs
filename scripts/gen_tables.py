#! /usr/bin/env python3

"""
Script for generating the pre-computed tables used by microfft:
  - radix-2 FFT twiddle tables
  - bit reversal tables

Used to create the file `src/tables.rs`.
"""

import argparse
import cmath
import math


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("N", type=int, help="Max FFT size")
    return parser.parse_args()


def iter_n(max_n):
    n = 1
    while n <= max_n:
        yield n
        n *= 2


def emit_uses():
    print("use num_complex::Complex32;")
    print()


def emit_twiddle(max_n):
    print("#[allow(clippy::approx_constant)]")
    print("#[allow(clippy::excessive_precision)]")
    print("#[allow(clippy::unreadable_literal)]")
    print("pub(crate) const TWIDDLE: &[&[Complex32]] = &[")
    for n in iter_n(max_n):
        emit_twiddle_table(n)
    print("];")
    print()


def emit_twiddle_table(n):
    print("    &[")
    for k in range(n // 2):
        twiddle = cmath.exp(-2j * math.pi * k / n)
        print(f"        Complex32::new({twiddle.real}, {twiddle.imag}),")
    print("    ],")


def emit_bitrev(max_n):
    print("pub(crate) const BITREV: &[&[u16]] = &[")
    for n in iter_n(max_n):
        emit_bitrev_table(n)
    print("];")
    print()


def emit_bitrev_table(n):
    print("    &[")
    nbits = int(math.log2(n))
    for i in range(n):
        rev = reverse_bits(i, nbits)
        entry = rev if rev > i else i
        print(f"        {entry},")
    print("    ],")


def reverse_bits(num, nbits):
    reverse = 0
    for i in range(nbits):
        if num & (1 << i):
            reverse |= 1 << (nbits - 1 - i)
    return reverse


def main():
    args = parse_args()
    emit_uses()
    emit_twiddle(args.N)
    emit_bitrev(args.N)


if __name__ == "__main__":
    main()
