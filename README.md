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
  Time (mean ± σ):      4.326 s ±  0.072 s    [User: 18.813 s, System: 5.873 s]
  Range (min … max):    4.209 s …  4.436 s    10 runs
```

```
# old default (scudo)
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     69.909 s ±  1.920 s    [User: 107.816 s, System: 18.912 s]
  Range (min … max):   67.537 s … 73.194 s    10 runs
```

```
# musl built with --with-malloc=mallocng
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     32.669 s ±  1.752 s    [User: 99.946 s, System: 69.685 s]
  Range (min … max):   30.495 s … 36.123 s    10 runs
```

```
# musl built with --with-malloc=oldmalloc
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     89.478 s ±  0.406 s    [User: 214.957 s, System: 226.522 s]
  Range (min … max):   88.767 s … 90.021 s    10 runs
```

```
# old default (scudo)
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     19.840 s ±  0.803 s    [User: 47.922 s, System: 13.896 s]
  Range (min … max):   18.742 s … 21.073 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/mimalloc/build/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      5.222 s ±  0.104 s    [User: 19.823 s, System: 6.596 s]
  Range (min … max):    5.096 s …  5.416 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/snmalloc/build/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      3.621 s ±  0.082 s    [User: 16.906 s, System: 4.160 s]
  Range (min … max):    3.528 s …  3.766 s    10 runs
```

#### `-n 4` (= CPU cores)

```
# new default (mimalloc)
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      2.833 s ±  0.095 s    [User: 6.724 s, System: 2.329 s]
  Range (min … max):    2.677 s …  2.972 s    10 runs
```

```
# old default (scudo)
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):     21.300 s ±  0.679 s    [User: 30.114 s, System: 4.826 s]
  Range (min … max):   20.338 s … 22.505 s    10 runs
```

```
# musl built with --with-malloc=mallocng
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):     13.886 s ±  0.135 s    [User: 29.156 s, System: 13.882 s]
  Range (min … max):   13.691 s … 14.055 s    10 runs
```

```
# musl built with --with-malloc=oldmalloc
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):     14.604 s ±  0.099 s    [User: 49.155 s, System: 4.059 s]
  Range (min … max):   14.492 s … 14.811 s    10 runs
```

```
# old default (scudo)
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      5.626 s ±  0.401 s    [User: 12.570 s, System: 2.867 s]
  Range (min … max):    4.923 s …  6.077 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/mimalloc/build/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      3.223 s ±  0.025 s    [User: 6.884 s, System: 2.589 s]
  Range (min … max):    3.185 s …  3.257 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/snmalloc/build/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      1.771 s ±  0.080 s    [User: 5.422 s, System: 0.976 s]
  Range (min … max):    1.633 s …  1.878 s    10 runs
```

### Archlinux

#### `-n 8` (default)

```
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      5.274 s ±  0.070 s    [User: 19.899 s, System: 7.920 s]
  Range (min … max):    5.160 s …  5.421 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     24.422 s ±  0.745 s    [User: 57.003 s, System: 16.837 s]
  Range (min … max):   23.142 s … 25.449 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):     19.029 s ±  0.151 s    [User: 44.179 s, System: 11.964 s]
  Range (min … max):   18.860 s … 19.343 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      5.248 s ±  0.065 s    [User: 18.493 s, System: 6.848 s]
  Range (min … max):    5.160 s …  5.352 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      4.060 s ±  0.100 s    [User: 17.137 s, System: 5.042 s]
  Range (min … max):    3.870 s …  4.220 s    10 runs
```

#### `-n 4` (= CPU cores)

```
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      3.000 s ±  0.089 s    [User: 6.731 s, System: 2.697 s]
  Range (min … max):    2.847 s …  3.119 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      7.629 s ±  0.130 s    [User: 15.287 s, System: 4.828 s]
  Range (min … max):    7.424 s …  7.768 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      6.522 s ±  0.101 s    [User: 13.086 s, System: 3.534 s]
  Range (min … max):    6.328 s …  6.637 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      3.204 s ±  0.134 s    [User: 6.527 s, System: 2.682 s]
  Range (min … max):    3.002 s …  3.493 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      1.883 s ±  0.050 s    [User: 5.483 s, System: 1.062 s]
  Range (min … max):    1.786 s …  1.945 s    10 runs
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
| Chimera (new default) | 0.7697 |
| Chimera (old default) | 1.65444 |
| Chimera (old default / no release) | 1.77762 |
| Chimera (`mallocng`) | 1.18592 |
| Chimera (`oldmalloc`) | 3.08846 |
| Chimera (`libmimalloc-secure`) | 0.81672 |
| Chimera (`libsnmallocshim-checks`) | 1.03064 |
| Arch (glibc) | 0.88617 |
| Arch (`libscudo`) | 1.61366 |
| Arch (`libscudo` / no release) | 1.47073 |
| Arch (`libmimalloc-secure`) | 0.82566 |
| Arch (`libsnmallocshim-checks`) | 1.08685 |

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
| Chimera (new default) | 2.53 | 4.66 |
| Chimera (old default) | 3.32 | 6.34 |
| Chimera (old default / no release) | 3.62 | 7.03 |
| Chimera (`mallocng`) | 2.86 | 5.52 |
| Chimera (`oldmalloc`) | 3.09 | 5.88 |
| Chimera (`libmimalloc-secure`) | 3.57 | 6.50 |
| Chimera (`libsnmallocshim-checks`) | 4.02 | 8.03 |
| Arch (glibc) | 3.09 | 6.15 |
| Arch (`libscudo`) | 3.23 | 6.28 |
| Arch (`libscudo` / no release) | 3.56 | 6.76 |
| Arch (`libmimalloc-secure`) | 3.53 | 6.92 |
| Arch (`libsnmallocshim-checks`) | 4.33 | 7.81 |
