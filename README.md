# Allocator performance test

This is an allocator performance test using a real-world load.

This is not a fully manufactured benchmark.

## Building

With nightly rust toolchain:
```
cargo build --release
```

It's possible to build with a stable toolchain like this:

```
RUSTC_BOOTSTRAP=1 cargo build --release
```

To change the parallelization level, change the value of `SPAWN_CHUNK_SZ` (set to 8) in `src/lib.rs`.
And if going beyond 16 parallel tasks is required, changing the value
`ASYNC_GLOBAL_EXECUTOR_THREADS` is set to in `src/bin/alloc-perf-test.rs` is also required.

The code here is a trimmed down and reduced version of a larger code base.
So it may look weird(er) in places.

## Usage

```
% ./target/release/alloc-perf-test
Usage: alloc-perf-test <COMMAND>

Commands:
  test-alloc-perf
  gen-data
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```
% ./target/release/alloc-perf-test gen-data --help
Usage: alloc-perf-test gen-data [OPTIONS]

Options:
  -n <N>       number of subs [default: 8]
  -s <SZ>      rough size of generated data relative to the default (SZ/DEF)^2 [default: 100]
  -h, --help   Print help
```

```
% ./target/release/alloc-perf-test test-alloc-perf --help
Usage: alloc-perf-test test-alloc-perf [OPTIONS]

Options:
  -n <N>      number of subs [default: 8]
  -h, --help  Print help
```

**Note**: `-n` value in `test-alloc-perf` should be equal or less of the `-n` value used in `gen-data`.

**Note 2**: Randomization and variation is involved when generating data, so two generated
data sets with the same settings won't give matching perf numbers, but the performance characteristics
shouldn't change.


## Test Results

[Related discussion](https://github.com/chimera-linux/cports/discussions/2480).

A similar test on a Laptop with half-sized data set is at [laptop_bench.md](./laptop_bench.md).

Tested on a system with i7-7700K processor (4 cores/8 threads) with maximum frequency limited to 3900MHz, fast NVMe storage, and plenty of RAM space to spare.

Generated data with defaults (`-n 8 -s 100`):

```
./target/release/alloc-perf-test gen-data
```

### Chimera (container)

#### `-n 8` (default)

```
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     72.055 s ±  1.265 s    [User: 123.700 s, System: 18.988 s]
  Range (min … max):   70.321 s … 73.915 s    10 runs
```

```
# musl built with --with-malloc=mallocng
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     64.292 s ±  0.652 s    [User: 212.194 s, System: 150.035 s]
  Range (min … max):   63.081 s … 65.057 s    10 runs
```

```
# musl built with --with-malloc=oldmalloc
  Time (mean ± σ):     151.351 s ±  4.249 s    [User: 400.331 s, System: 373.181 s]
  Range (min … max):   148.193 s … 162.803 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     21.129 s ±  1.603 s    [User: 63.501 s, System: 13.188 s]
  Range (min … max):   18.665 s … 23.663 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/mimalloc/build/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      6.332 s ±  0.084 s    [User: 27.682 s, System: 6.665 s]
  Range (min … max):    6.235 s …  6.498 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/snmalloc/build/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      4.187 s ±  0.125 s    [User: 21.783 s, System: 3.589 s]
  Range (min … max):    4.010 s …  4.394 s    10 runs
```

#### `-n 4` (= CPU cores)

```
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):     22.503 s ±  0.602 s    [User: 34.273 s, System: 4.796 s]
  Range (min … max):   21.830 s … 23.709 s    10 runs
```

```
# musl built with --with-malloc=mallocng
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):     28.775 s ±  0.474 s    [User: 66.112 s, System: 30.042 s]
  Range (min … max):   28.023 s … 29.676 s    10 runs
```

```
# musl built with --with-malloc=oldmalloc
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):     24.663 s ±  0.245 s    [User: 85.588 s, System: 5.029 s]
  Range (min … max):   24.264 s … 25.015 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      7.095 s ±  0.319 s    [User: 17.341 s, System: 2.771 s]
  Range (min … max):    6.559 s …  7.610 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/mimalloc/build/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      3.797 s ±  0.072 s    [User: 8.899 s, System: 2.579 s]
  Range (min … max):    3.681 s …  3.900 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/snmalloc/build/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      2.214 s ±  0.062 s    [User: 6.544 s, System: 0.930 s]
  Range (min … max):    2.097 s …  2.326 s    10 runs
```

### Archlinux

#### `-n 8` (default)

```
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      9.544 s ±  1.735 s    [User: 33.877 s, System: 13.351 s]
  Range (min … max):    7.889 s … 13.886 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     23.615 s ±  0.151 s    [User: 63.080 s, System: 16.143 s]
  Range (min … max):   23.382 s … 23.913 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     19.844 s ±  0.082 s    [User: 52.574 s, System: 12.308 s]
  Range (min … max):   19.737 s … 19.992 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      6.097 s ±  0.099 s    [User: 25.514 s, System: 6.884 s]
  Range (min … max):    5.893 s …  6.224 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      4.380 s ±  0.186 s    [User: 22.392 s, System: 4.062 s]
  Range (min … max):    4.178 s …  4.803 s    10 runs
```

#### `-n 4` (= CPU cores)

```
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      6.711 s ±  1.667 s    [User: 13.896 s, System: 5.977 s]
  Range (min … max):    4.423 s … 10.467 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      7.951 s ±  0.110 s    [User: 17.179 s, System: 4.435 s]
  Range (min … max):    7.724 s …  8.099 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      6.966 s ±  0.059 s    [User: 15.262 s, System: 3.455 s]
  Range (min … max):    6.873 s …  7.062 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      3.708 s ±  0.075 s    [User: 8.270 s, System: 2.665 s]
  Range (min … max):    3.599 s …  3.810 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      2.214 s ±  0.034 s    [User: 6.601 s, System: 1.022 s]
  Range (min … max):    2.156 s …  2.273 s    10 runs
```
