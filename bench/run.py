#! /usr/bin/env python3

import os
import string
import time
from pathlib import Path
from subprocess import Popen, DEVNULL


def start_openocd():
    cmd = ["openocd", "-f", "interface/stlink-v2-1.cfg", "-f", "target/stm32f3x.cfg"]
    proc = Popen(cmd, stdout=DEVNULL, stderr=DEVNULL)
    time.sleep(1)
    return proc


def run_bench(name, size):
    output_file = Path("itm.txt")
    output_file.touch(exist_ok=False)

    features = f"{name},n-{size}"
    cmd = ["cargo", "run", "--release", "--features", features]

    proc = Popen(cmd, stdout=DEVNULL, stderr=DEVNULL)
    try:
        output = wait_for_file(output_file)
    finally:
        proc.terminate()
        output_file.unlink()

    cycles = parse_output(output)
    print(f"({name}, {size}): {cycles}")


def wait_for_file(path):
    while True:
        contents = path.read_bytes()
        if contents:
            return contents
        time.sleep(0.1)


def parse_output(output):
    chars = (chr(b) for b in output)
    printable = (c for c in chars if c in string.printable)
    return "".join(printable)


def run_benches():
    for i in range(2, 13):
        run_bench("microfft-c", 2 ** i)
    for i in range(2, 13):
        run_bench("microfft-r", 2 ** i)
    for i in range(2, 10):
        run_bench("fourier-c", 2 ** i)


def main():
    bench_path = Path(__file__).resolve().parent
    os.chdir(bench_path)

    openocd = start_openocd()

    try:
        run_benches()
    finally:
        openocd.terminate()

    openocd.terminate()


if __name__ == "__main__":
    main()
