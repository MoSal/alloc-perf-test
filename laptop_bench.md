## Laptop Benchmarks

Laptop with i5-9300H CPU (4 cores/8 threads), 8GiB RAM (IDLE with < 1GiB resident).

Testing on smaller data set here generated with (for roughly ~50% size):

```
./target/release/alloc-perf-test gen-data -s 71
```

### Chimera

#### `-n 8` (default)

```
Benchmark 1: ./alloc-perf-test test-alloc-perf
  Time (mean ± σ):     23.973 s ±  0.684 s    [User: 48.865 s, System: 18.775 s]
  Range (min … max):   22.632 s … 24.769 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 ./alloc-perf-test test-alloc-perf
  Time (mean ± σ):      9.021 s ±  0.732 s    [User: 34.034 s, System: 17.093 s]
  Range (min … max):    7.695 s …  9.834 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/chimera/libmimalloc-secure.so ./alloc-perf-test test-alloc-perf
  Time (mean ± σ):      3.693 s ±  0.078 s    [User: 17.534 s, System: 4.497 s]
  Range (min … max):    3.568 s …  3.793 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/chimera/libsnmallocshim-checks.so ./alloc-perf-test test-alloc-perf
  Time (mean ± σ):      3.117 s ±  0.050 s    [User: 16.387 s, System: 4.484 s]
  Range (min … max):    3.026 s …  3.192 s    10 runs
```

#### `-n 4` (= CPU cores)

```
  Time (mean ± σ):      7.990 s ±  0.204 s    [User: 13.926 s, System: 2.919 s]
  Range (min … max):    7.671 s …  8.373 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 ./alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      3.453 s ±  0.094 s    [User: 8.724 s, System: 1.896 s]
  Range (min … max):    3.309 s …  3.599 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/chimera/libmimalloc-secure.so ./alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      2.142 s ±  0.063 s    [User: 5.196 s, System: 1.541 s]
  Range (min … max):    2.047 s …  2.248 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/chimera/libsnmallocshim-checks.so ./alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      1.542 s ±  0.023 s    [User: 4.528 s, System: 1.101 s]
  Range (min … max):    1.504 s …  1.579 s    10 runs
```

### Archlinux

#### `-n 8` (default)

```
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      6.065 s ±  0.734 s    [User: 22.489 s, System: 10.241 s]
  Range (min … max):    5.169 s …  7.156 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      8.520 s ±  0.114 s    [User: 30.418 s, System: 10.247 s]
  Range (min … max):    8.388 s …  8.722 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      7.301 s ±  0.071 s    [User: 27.467 s, System: 8.293 s]
  Range (min … max):    7.203 s …  7.462 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      3.709 s ±  0.055 s    [User: 16.696 s, System: 5.191 s]
  Range (min … max):    3.635 s …  3.784 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf
  Time (mean ± σ):      3.185 s ±  0.077 s    [User: 16.700 s, System: 4.524 s]
  Range (min … max):    3.075 s …  3.326 s    10 runs
```

#### `-n 4` (= CPU cores)

```
Benchmark 1: ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      3.861 s ±  0.725 s    [User: 7.907 s, System: 3.538 s]
  Range (min … max):    2.616 s …  4.975 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      3.667 s ±  0.084 s    [User: 8.322 s, System: 2.473 s]
  Range (min … max):    3.509 s …  3.777 s    10 runs
```

```
Benchmark 1: SCUDO_OPTIONS=release_to_os_interval_ms=-1 LD_PRELOAD=/tmp/libscudo.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      3.134 s ±  0.055 s    [User: 7.449 s, System: 2.016 s]
  Range (min … max):    3.073 s …  3.232 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libmimalloc-secure.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      2.038 s ±  0.042 s    [User: 4.769 s, System: 1.676 s]
  Range (min … max):    1.983 s …  2.105 s    10 runs
```

```
Benchmark 1: LD_PRELOAD=/tmp/libsnmallocshim-checks.so ./target/release/alloc-perf-test test-alloc-perf -n 4
  Time (mean ± σ):      1.575 s ±  0.038 s    [User: 4.537 s, System: 1.146 s]
  Range (min … max):    1.532 s …  1.640 s    10 runs
```
