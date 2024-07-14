# fft-benchmark

This benchmark compares the performance of various fast Fourier transform libraries for Rust.

Libraries included:
- [`fftw`](https://crates.io/crates/fftw) v0.8.0 (bindings to [FFTW](http://www.fftw.org) v3.3.8)
- [`rustfft`](https://crates.io/crates/rustfft) v6.2.0
- [`phastft`](https://crates.io/crates/phastft) v0.2.1

## Results

All measurements were taken on an M1 MacBook Pro with `rustc 1.80.0-nightly (d2d24e395 2024-05-03)`. FFTW was compiled with `Apple clang version 15.0.0 (clang-1500.0.40.1)`. For time taken, the first number is for the forward FFT, and the second number is for the inverse FFT. Median times are listed. For each row, the fastest time and the least total memory allocated are highlighted in **bold**.

### `f32`

| Sequence length |     `fftw` time     |       `rustfft` time        |       `phastft` time        | `rustfft` memory | `phastft` memory |
| :-------------: | :-----------------: | :-------------------------: | :-------------------------: | :--------------: | :--------------: |
|        8        | 1.374 μs / 1.458 μs |     83.69 ns / 83.19 ns     | **82.68 ns** / **69.33 ns** |      368 B       |     **32 B**     |
|       16        | 1.405 μs / 1.416 μs | **124.6 ns** / **94.09 ns** |     137.7 ns / 137.7 ns     |      624 B       |     **96 B**     |
|       32        | 1.457 μs / 1.479 μs | **119.5 ns** / **116.2 ns** |     248.3 ns / 240.5 ns     |     1.152 KB     |    **224 B**     |
|       64        | 3.291 μs / 3.333 μs |     562.1 ns / 556.9 ns     | **419.0 ns** / **395.4 ns** |     3.808 KB     |    **480 B**     |
|       128       | 3.541 μs / 3.541 μs |     1.093 μs / 1.093 μs     | **681.9 ns** / **687.1 ns** |     6.368 KB     |    **992 B**     |
|       256       | 4.124 μs / 4.083 μs |     1.958 μs / 1.958 μs     | **1.291 μs** / **1.312 μs** |     11.48 KB     |   **2.016 KB**   |
|       512       | 5.249 μs / 5.290 μs |     3.915 μs / 3.916 μs     | **2.457 μs** / **2.479 μs** |     21.72 KB     |   **4.064 KB**   |
|      1024       | 7.291 μs / 7.374 μs |     7.707 μs / 7.624 μs     | **4.749 μs** / **4.832 μs** |     42.20 KB     |   **8.160 KB**   |
|      2048       | 11.74 μs / 11.83 μs |     22.87 μs / 15.91 μs     | **9.291 μs** / **9.457 μs** |     83.16 KB     |   **16.35 KB**   |
|      4096       | 21.20 μs / 21.16 μs |     45.33 μs / 31.66 μs     | **18.62 μs** / **18.87 μs** |     165.0 KB     |   **32.73 KB**   |
|      8192       | 45.62 μs / 120.7 μs |     84.08 μs / 66.45 μs     | **37.87 μs** / **38.24 μs** |     328.9 KB     |   **65.50 KB**   |
|      16384      | 101.1 μs / 180.0 μs |     156.7 μs / 225.2 μs     | **81.56 μs** / **83.24 μs** |     656.6 KB     |   **131.0 KB**   |
|      32768      | 333.3 μs / 417.4 μs |     289.6 μs / 289.7 μs     | **249.2 μs** / **257.1 μs** |     1.311 MB     |   **262.1 KB**   |
|      65536      | 806.4 μs / 895.0 μs |     551.4 μs / 633.6 μs     | **524.0 μs** / **542.9 μs** |     2.622 MB     |   **524.2 KB**   |
|     131072      | 1.675 ms / 1.703 ms |     1.211 ms / 1.296 ms     | **1.069 ms** / **1.176 ms** |     5.244 MB     |   **1.048 MB**   |
|     262144      | 3.487 ms / 3.566 ms |     2.432 ms / 2.482 ms     | **2.270 ms** / **2.366 ms** |     10.48 MB     |   **2.097 MB**   |
|     524288      | 6.261 ms / 6.125 ms |     5.804 ms / 5.834 ms     | **4.697 ms** / **4.874 ms** |     20.97 MB     |   **4.194 MB**   |
|     1048576     | 14.20 ms / 13.13 ms |     12.49 ms / 12.58 ms     | **10.21 ms** / **10.25 ms** |     41.94 MB     |   **8.388 MB**   |
|     2097152     | 41.06 ms / 40.01 ms |     23.81 ms / 23.77 ms     | **21.20 ms** / **21.34 ms** |     83.88 MB     |   **16.77 MB**   |
|     4194304     | 90.12 ms / 88.03 ms | **49.32 ms** / **49.27 ms** |     49.82 ms / 50.64 ms     |     167.7 MB     |   **33.55 MB**   |
|     8388608     | 193.3 ms / 192.9 ms | **103.9 ms** / **103.6 ms** |     109.0 ms / 108.5 ms     |     335.5 MB     |   **67.10 MB**   |

### `f64`

| Sequence length |     `fftw` time     |       `rustfft` time        |       `phastft` time        | `rustfft` memory | `phastft` memory |
| :-------------: | :-----------------: | :-------------------------: | :-------------------------: | :--------------: | :--------------: |
|        8        | 1.499 μs / 1.582 μs |     124.6 ns / 124.6 ns     | **82.68 ns** / **79.75 ns** |      320 B       |     **64 B**     |
|       16        | 1.468 μs / 1.593 μs | **124.6 ns** / **91.49 ns** |     144.2 ns / 146.8 ns     |      480 B       |    **192 B**     |
|       32        | 1.624 μs / 1.624 μs | **165.6 ns** / **129.9 ns** |     248.3 ns / 253.5 ns     |      816 B       |    **448 B**     |
|       64        | 3.540 μs / 3.416 μs |     619.4 ns / 619.4 ns     |   **424.1 ns / 431.9 ns**   |     3.968 KB     |    **960 B**     |
|       128       | 3.749 μs / 3.749 μs |     1.145 μs / 1.145 μs     |   **807.0 ns / 822.5 ns**   |     7.040 KB     |   **1.984 KB**   |
|       256       | 4.291 μs / 4.291 μs |     2.166 μs / 2.166 μs     |   **1.520 μs / 1.541 μs**   |     13.18 KB     |   **4.032 KB**   |
|       512       | 5.749 μs / 5.707 μs |     4.374 μs / 4.333 μs     |   **2.937 μs / 2.999 μs**   |     25.47 KB     |   **8.128 KB**   |
|      1024       | 7.749 μs / 7.749 μs |     8.624 μs / 8.624 μs     |   **5.791 μs / 5.915 μs**   |     50.04 KB     |   **16.32 KB**   |
|      2048       | 12.54 μs / 12.74 μs |     18.49 μs / 18.45 μs     |   **11.66 μs / 11.91 μs**   |     99.20 KB     |   **32.70 KB**   |
|      4096       | 71.49 μs / 62.47 μs |     39.29 μs / 38.70 μs     |   **23.95 μs / 24.33 μs**   |     197.5 KB     |   **65.47 KB**   |
|      8192       | 102.3 μs / 124.1 μs |     83.70 μs / 84.02 μs     |   **54.02 μs / 54.12 μs**   |     394.1 KB     |   **131.0 KB**   |
|      16384      | 259.8 μs / 257.3 μs |     180.5 μs / 177.6 μs     |   **127.4 μs / 127.4 μs**   |     787.3 KB     |   **262.0 KB**   |
|      32768      | 502.6 μs / 409.0 μs |     382.2 μs / 387.7 μs     |   **348.2 μs / 348.4 μs**   |     1.573 MB     |   **524.2 KB**   |
|      65536      | 953.5 μs / 966.9 μs | **754.4 μs** / **774.5 μs** |     802.1 μs / 802.5 μs     |     3.146 MB     |   **1.048 MB**   |
|     131072      | 2.001 ms / 1.923 ms |     1.769 ms / 1.768 ms     | **1.593 ms** / **1.600 ms** |     6.292 MB     |   **2.097 MB**   |
|     262144      | 4.006 ms / 3.924 ms |     3.499 ms / 3.490 ms     | **3.355 ms** / **3.317 ms** |     12.58 MB     |   **4.194 MB**   |
|     524288      | 7.911 ms / 7.499 ms |     9.427 ms / 9.237 ms     | **7.148 ms** / **7.149 ms** |     25.16 MB     |   **8.388 MB**   |
|     1048576     | 20.08 ms / 19.62 ms |     16.89 ms / 16.86 ms     | **15.10 ms** / **15.36 ms** |     50.33 MB     |   **16.77 MB**   |
|     2097152     | 48.34 ms / 47.62 ms | **35.59 ms** / **35.61 ms** |     36.69 ms / 36.84 ms     |     100.6 MB     |   **33.55 MB**   |
|     4194304     | 107.9 ms / 106.9 ms | **74.62 ms** / **74.55 ms** |     78.20 ms / 78.40 ms     |     201.3 MB     |   **67.10 MB**   |
|     8388608     | 217.3 ms / 215.3 ms | **149.3 ms** / **150.5 ms** |     169.6 ms / 169.6 ms     |     402.6 MB     |   **134.2 MB**   |

## Notes

- `fftw` consists of Rust bindings to the FFTW library, which is written in C. `rustfft` and `phastft` are pure Rust.
- The benchmarking library used, [`divan`](https://crates.io/crates/divan), cannot track memory allocations in C, so the amounts of memory allocated by `fftw` are not included in the benchmark results.
- Benchmarked code does not contain multi-threading; computations are performed on a single thread. However, `phastft` can perform [multi-threaded bit reversal](https://docs.rs/phastft/0.2.1/phastft/options/struct.Options.html#structfield.multithreaded_bit_reversal). When enabled, `phastft` is faster than `rustfft` at the largest sequence lengths tested.

## Conclusion

`phastft` was the fastest in most tests, but `rustfft` was the fastest on the smallest and largest sequence lengths tested. `phastft` allocated *significantly* less memory than `rustfft`. `fftw` was consistently slower than both `rustfft` and `phastft`.

`phastft` currently has a few limitations:
- requires nightly Rust due to usage of [portable SIMD](https://doc.rust-lang.org/1.76.0/std/simd/index.html)
- does not support computing Fourier transforms for sequences with lengths that are not a power of 2

If these don't matter for your use case, `phastft` is an excellent choice.