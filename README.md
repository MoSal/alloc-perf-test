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

### Allocation stats

A `stats_alloc` feature exists which, when enabled, will print overall allocation stats after a run is finished.
This should incur less performance overhead compared to using other tools like `DHAT`.
Since this won't be used with maximum performance in mind, and for quick iteration, it might be a good idea to use
this with quick builds that use `cranelift` for code generation:

```
cargo build --features=stats_alloc --profile release-dev-cl
```

Binary built will be at `./target/release-dev-cl/alloc-perf-test`.

This requires `cranelift` to be available in as a code-generator in the Rust tooling. This is done usually by installing a `rustup` component.

### Parallelism

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

Tested on a system with i7-7700K processor (4 cores/8 threads) with maximum frequency limited to 3900MHz, fast NVMe storage, and plenty of RAM space to spare.

Generated data with defaults (`-n 8 -s 100`):

***Chimera (new default)*** tests added using musl-1.2.5_git20240705-r3 which is patched to use **mimalloc** with a custom secure/hardened profile by default.

```
./target/release/alloc-perf-test gen-data
```

Below numbers belong to the currently checked out branch.

### Chimera (container)

#### `-n 8` (default)

```
# new default (mimalloc)
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      4.160 s ±  0.050 s    [User: 18.610 s, System: 6.020 s]
  Range (min … max):    4.061 s …  4.227 s    10 runs
```

```
# old default (scudo)
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     54.290 s ±  2.319 s    [User: 95.083 s, System: 19.967 s]
  Range (min … max):   50.635 s … 58.560 s    10 runs
```

```
# musl built with --with-malloc=mallocng
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     27.800 s ±  0.254 s    [User: 90.072 s, System: 62.414 s]
  Range (min … max):   27.281 s … 28.139 s    10 runs
```

```
# musl built with --with-malloc=oldmalloc
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     73.854 s ±  0.765 s    [User: 174.749 s, System: 202.590 s]
  Range (min … max):   73.045 s … 75.275 s    10 runs
```

```
# old default (scudo)
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     13.327 s ±  0.999 s    [User: 39.456 s, System: 15.038 s]
  Range (min … max):   11.791 s … 14.993 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/mimalloc/build/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      4.654 s ±  0.097 s    [User: 19.245 s, System: 6.860 s]
  Range (min … max):    4.550 s …  4.854 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/snmalloc/build/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      3.599 s ±  0.097 s    [User: 16.512 s, System: 4.806 s]
  Range (min … max):    3.519 s …  3.824 s    10 runs
```

#### `-n 4` (= CPU cores)

```
# new default (mimalloc)
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      2.741 s ±  0.121 s    [User: 6.679 s, System: 2.472 s]
  Range (min … max):    2.597 s …  3.003 s    10 runs
```

```
# old default (scudo)
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):     16.500 s ±  0.400 s    [User: 25.316 s, System: 4.900 s]
  Range (min … max):   16.048 s … 17.112 s    10 runs
```

```
# musl built with --with-malloc=mallocng
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):     12.610 s ±  0.134 s    [User: 27.435 s, System: 13.455 s]
  Range (min … max):   12.394 s … 12.792 s    10 runs
```

```
# musl built with --with-malloc=oldmalloc
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):     14.247 s ±  0.130 s    [User: 49.907 s, System: 4.658 s]
  Range (min … max):   14.008 s … 14.398 s    10 runs
```

```
# old default (scudo)
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      4.596 s ±  0.257 s    [User: 11.570 s, System: 3.271 s]
  Range (min … max):    4.275 s …  4.986 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/mimalloc/build/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      2.963 s ±  0.055 s    [User: 6.625 s, System: 2.810 s]
  Range (min … max):    2.883 s …  3.063 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/snmalloc/build/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      1.822 s ±  0.069 s    [User: 5.483 s, System: 1.230 s]
  Range (min … max):    1.738 s …  1.922 s    10 runs
```

### Archlinux

#### `-n 8` (default)

```
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      4.912 s ±  0.068 s    [User: 19.778 s, System: 9.071 s]
  Range (min … max):    4.801 s …  5.013 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     16.353 s ±  1.365 s    [User: 49.113 s, System: 17.179 s]
  Range (min … max):   15.243 s … 19.796 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     11.925 s ±  0.196 s    [User: 36.516 s, System: 12.815 s]
  Range (min … max):   11.577 s … 12.165 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      4.556 s ±  0.083 s    [User: 17.790 s, System: 7.102 s]
  Range (min … max):    4.441 s …  4.717 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      3.803 s ±  0.079 s    [User: 16.874 s, System: 4.986 s]
  Range (min … max):    3.719 s …  3.958 s    10 runs
```

#### `-n 4` (= CPU cores)

```
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      2.834 s ±  0.072 s    [User: 6.561 s, System: 2.945 s]
  Range (min … max):    2.734 s …  2.976 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      5.467 s ±  0.077 s    [User: 13.197 s, System: 4.817 s]
  Range (min … max):    5.386 s …  5.636 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      4.800 s ±  0.065 s    [User: 11.476 s, System: 3.687 s]
  Range (min … max):    4.679 s …  4.888 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      2.838 s ±  0.049 s    [User: 5.988 s, System: 2.840 s]
  Range (min … max):    2.771 s …  2.913 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      1.947 s ±  0.053 s    [User: 5.354 s, System: 1.326 s]
  Range (min … max):    1.883 s …  2.024 s    10 runs
```

###  Parallelism Performance Degradation

Below, we record the effect of increased parallelism from `-n 4` to `-n 8` (default).

Numbers are calculated as:

```
time(n8) / time(n4) / (size(n8) / size(n4))
```

Mean numbers are used for time.

In the tested dataset, the data size of `-n 8`/`-n 4` is `~1.98382`.

Numbers significantly bigger than `1.00` may indicate degraded performance from increased parallelism.

Numbers significantly smaller than `1.00` may indicate non-optimal performance with less parallelism.

Incidentally (or maybe not so), the fastest allocator is also the closest to `1.00`.

| Test | Degradation |
|:----:|:----:|
| Chimera (new default) | 0.76504 |
| Chimera (old default) | 1.65857 |
| Chimera (old default / no release) | 1.46167 |
| Chimera (`mallocng`) | 1.11129 |
| Chimera (`oldmalloc`) | 2.61305 |
| Chimera (`libmimalloc-secure`) | 0.79176 |
| Chimera (`libsnmallocshim-checks`) | 0.99571 |
| Arch (glibc) | 0.87369 |
| Arch (`libscudo`) | 1.50781 |
| Arch (`libscudo` / no release) | 1.25232 |
| Arch (`libmimalloc-secure`) | 0.80922 |
| Arch (`libsnmallocshim-checks`) | 0.98460 |

### Max memory (RSS) usage

Picking the largest number in three runs using:

```
for _ in {1..3}; do >/tmp/lines;  while [[ -z `cat /tmp/lines` ]] || pidof alloc-perf-test &>/dev/null; do sleep 0.01; grep '^VmRSS' /proc/`pidof alloc-perf-test`/status >>/tmp/lines 2>/dev/null; done; sort -h /tmp/lines | tail -1; done
```

Runs done like this:

```
for _ in {1..3}; do [env_vars] ./target/release/alloc-perf-test test-alloc-perf [-n 4]; sleep 1 ; done
```

***Numbers in GiB units.***

| Test | `-n 4` | `-n 8` (default) |
|:----:|:----:|:--------:|
| Chimera (new default) | 2.70 | 5.16 |
| Chimera (old default) | 3.86 | 7.23 |
| Chimera (old default / no release) | 4.14 | 7.78 |
| Chimera (`mallocng`) | 3.27 | 6.21 |
| Chimera (`oldmalloc`) | 3.62 | 6.76 |
| Chimera (`libmimalloc-secure`) | 3.77 | 6.92 |
| Chimera (`libsnmallocshim-checks`) | 4.02 | 8.38 |
| Arch (glibc) | 3.66 | 7.21 |
| Arch (`libscudo`) | 3.73 | 7.18 |
| Arch (`libscudo` / no release) | 4.01 | 7.62 |
| Arch (`libmimalloc-secure`) | 3.54 | 7.00 |
| Arch (`libsnmallocshim-checks`) | 4.32 | 8.65 |
