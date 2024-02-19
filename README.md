# fft-benchmark

This benchmark compares the performance of various fast Fourier transform libraries for Rust.

Libraries included:
- [`fftw`](https://crates.io/crates/fftw) v0.8.0 (bindings to [FFTW](http://www.fftw.org) v3.3.8)
- [`rustfft`](https://crates.io/crates/rustfft) v6.2.0
- [`phastft`](https://crates.io/crates/phastft) v0.1.1

## Results

All measurements were taken on an M1 MacBook Pro with `rustc 1.78.0-nightly (6672c16af 2024-02-17)`. FFTW was compiled with `Apple clang version 15.0.0 (clang-1500.0.40.1)`. For time taken, the first number is for the forward FFT, and the second number is for the inverse FFT. Median times are listed. For each row, the fastest time and the least total memory allocated are highlighted in **bold**.

| Sequence length |         `fftw` time         |       `rustfft` time        |       `phastft` time        | `rustfft` memory | `phastft` memory |
| :-------------: | :-------------------------: | :-------------------------: | :-------------------------: | :--------------: | :--------------: |
|        8        |     1.749 μs / 1.749 μs     |  **165.6 ns** / **126 ns**  |     166.6 ns / 137.7 ns     |      320 B       |     **64 B**     |
|       16        |     1.749 μs / 1.812 μs     | **142.9 ns** / **141.6 ns** |     252.3 ns / 265.3 ns     |      480 B       |    **192 B**     |
|       32        |     1.832 μs / 1.749 μs     |  **166.6 ns** / **165 ns**  |     447.5 ns / 442.4 ns     |      816 B       |    **448 B**     |
|       64        |     4.208 μs / 3.916 μs     | **692.3 ns** / **655.9 ns** |      695 ns / 661.1 ns      |     3.968 KB     |    **960 B**     |
|       128       |     4.541 μs / 4.249 μs     |     1.228 μs / 1.197 μs     | **1.114 μs** / **1.041 μs** |     7.04 KB      |   **1.984 KB**   |
|       256       |     5.332 μs / 4.916 μs     |     2.29 μs / 2.166 μs      | **1.739 μs** / **1.718 μs** |     13.18 KB     |   **4.032 KB**   |
|       512       |     6.832 μs / 6.29 μs      |     4.582 μs / 4.583 μs     | **3.249 μs** / **3.124 μs** |     25.47 KB     |   **8.128 KB**   |
|      1024       |     8.979 μs / 8.333 μs     |     9.145 μs / 8.936 μs     | **6.207 μs** / **5.791 μs** |     50.04 KB     |   **16.32 KB**   |
|      2048       |     13.99 μs / 13.24 μs     |     19.54 μs / 18.52 μs     | **12.24 μs** / **11.54 μs** |     99.2 KB      |   **32.7 KB**    |
|      4096       | **23.33 μs** / **23.29 μs** |     36.95 μs / 41.87 μs     |     24.87 μs / 23.41 μs     |     197.5 KB     |   **65.47 KB**   |
|      8192       |   **52.58 μs** / 74.1 μs    |     77.04 μs / 81.43 μs     |   53.83 μs / **52.2 μs**    |     394.1 KB     |    **131 KB**    |
|      16384      |     196.8 μs / 191.3 μs     |     172.8 μs / 169.8 μs     | **120.2 μs** / **122.5 μs** |     787.3 KB     |    **262 KB**    |
|      32768      |     397.7 μs / 395.9 μs     |     389.4 μs / 343.4 μs     | **340.4 μs** / **342.9 μs** |     1.573 MB     |   **524.2 KB**   |
|      65536      |      905.6 μs / 936 μs      |     745.1 μs / 758.5 μs     | **670.6 μs** / **754.7 μs** |     3.146 MB     |   **1.048 MB**   |
|     131072      |     1.838 ms / 1.83 ms      |     1.639 ms / 1.661 ms     | **1.458 ms** / **1.506 ms** |     6.292 MB     |   **2.097 MB**   |
|     262144      |     3.849 ms / 3.918 ms     |     3.332 ms / 3.373 ms     | **3.035 ms** / **3.097 ms** |     12.58 MB     |   **4.194 MB**   |
|     524288      |     7.21 ms / 7.434 ms      |     9.08 ms / 9.301 ms      | **6.643 ms** / **6.739 ms** |     25.16 MB     |   **8.388 MB**   |
|     1048576     |     20.43 ms / 19.84 ms     |     16.25 ms / 16.3 ms      | **14.42 ms** / **14.49 ms** |     50.33 MB     |   **16.77 MB**   |
|     2097152     |     49.31 ms / 48.85 ms     |  **34 ms** / **34.15 ms**   |     35.43 ms / 35.5 ms      |     100.6 MB     |   **33.55 MB**   |
|     4194304     |     108.1 ms / 106.7 ms     | **71.94 ms** / **72.65 ms** |     76.26 ms / 76.53 ms     |     201.3 MB     |   **67.1 MB**    |
|     8388608     |     217.3 ms / 216.7 ms     | **156.3 ms** / **157.5 ms** |     166.8 ms / 166.7 ms     |     402.6 MB     |   **134.2 MB**   |

## Notes

- `fftw` consists of Rust bindings to the FFTW library, which is written in C. `rustfft` and `phastft` are pure Rust.
- The benchmarking library used, [`divan`](https://crates.io/crates/divan), cannot track memory allocations in C, so the amounts of memory allocated by `fftw` are not included in the benchmark results.
- Benchmarked code does not contain multi-threading; computations are performed on a single thread. However, `phastft` can perform [multi-threaded bit reversal](https://docs.rs/phastft/0.1.1/phastft/options/struct.Options.html#structfield.multithreaded_bit_reversal). When enabled, `phastft` is faster than `rustfft` at the largest sequence lengths tested.

## Conclusion

`phastft` was the fastest in most tests, but `rustfft` was the fastest on the smallest and largest sequence lengths tested. `phastft` allocated *significantly* less memory than `rustfft`. `fftw` was consistently slower than both `rustfft` and `phastft`.

`phastft` currently has several limitations:
- requires nightly Rust due to usage of [portable SIMD](https://doc.rust-lang.org/1.76.0/std/simd/index.html)
- does not support computing Fourier transforms for sequences with lengths that are not a power of 2; a workaround for this is to pad the sequence as necessary
- does not support `f32` for computing Fourier transforms, unlike `fftw` and `rustfft` which support both `f32` and `f64`

If these don't matter for your use case, `phastft` is an excellent choice.