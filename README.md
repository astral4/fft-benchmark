# fft-benchmark

This benchmark compares the performance of two fast Fourier transform libraries for Rust: [`rustfft`](https://crates.io/crates/rustfft) and [`phastft`](https://crates.io/crates/phastft).

## Results

All measurements were taken on an M1 MacBook Pro with `rustc 1.78.0-nightly (8ace7ea1f 2024-02-07)`. For time taken, the first number is for the forward FFT, and the second number is for the inverse FFT. Median times are listed. For each row, the fastest time and the least total memory allocated are highlighted in **bold**.

| Sequence length |       `rustfft` time        |       `phastft` time        | `rustfft` memory | `phastft` memory |
| :-------------: | :-------------------------: | :-------------------------: | :--------------: | :--------------: |
|        8        |     124.6 ns / 125.9 ns     | **112.9 ns** / **103.1 ns** |      320 B       |     **64 B**     |
|       16        | **124.6 ns** / **138.9 ns** |     240.5 ns / 204.1 ns     |      480 B       |    **192 B**     |
|       32        | **166.6 ns** / **163.7 ns** |     452.7 ns / 392.9 ns     |      816 B       |    **448 B**     |
|       64        |   **687.1 ns** / 655.9 ns   |    707.9 ns / **635 ns**    |     3.968 KB     |    **960 B**     |
|       128       |     1.249 μs / 1.166 μs     | **1.098 μs** / **999.6 ns** |     7.04 KB      |   **1.984 KB**   |
|       256       |     2.312 μs / 2.166 μs     | **1.791 μs** / **1.754 μs** |     13.18 KB     |   **4.032 KB**   |
|       512       |     4.583 μs / 4.333 μs     | **3.322 μs** / **3.02 μs**  |     25.47 KB     |   **8.128 KB**   |
|      1024       |     9.207 μs / 8.624 μs     | **6.208 μs** / **5.749 μs** |     50.04 KB     |   **16.32 KB**   |
|      2048       |     19.45 μs / 18.33 μs     | **11.81 μs** / **11.37 μs** |     99.2 KB      |   **32.7 KB**    |
|      4096       |     39.18 μs / 40.62 μs     | **24.66 μs** / **23.41 μs** |     197.5 KB     |   **65.47 KB**   |
|      8192       |     77.27 μs / 90.12 μs     | **53.87 μs** / **52.29 μs** |     394.1 KB     |    **131 KB**    |
|      16384      |     162.9 μs / 174.9 μs     | **124.2 μs** / **122.9 μs** |     787.3 KB     |    **262 KB**    |
|      32768      |   **341.7 μs** / 356.6 μs   |   354.7 μs / **355.5 μs**   |     1.573 MB     |   **524.2 KB**   |
|      65536      |   804.8 μs / **709.7 μs**   |   **752.8 μs** / 817.2 μs   |     3.146 MB     |   **1.048 MB**   |
|     131072      |     1.799 ms / 1.643 ms     | **1.629 ms** / **1.622 ms** |     6.292 MB     |   **2.097 MB**   |
|     262144      | **3.446 ms** / **3.397 ms** |     3.543 ms / 3.425 ms     |     12.58 MB     |   **4.194 MB**   |
|     524288      |     9.576 ms / 9.292 ms     | **7.745 ms** / **7.339 ms** |     25.16 MB     |   **8.388 MB**   |
|     1048576     |     18.01 ms / 16.42 ms     | **15.8 ms** / **15.86 ms**  |     50.33 MB     |   **16.77 MB**   |
|     2097152     | **37.45 ms** / **34.16 ms** |     38.81 ms / 36.82 ms     |     100.6 MB     |   **33.55 MB**   |
|     4194304     |     78.41 ms / 79.07 ms     | **65.42 ms** / **66.28 ms** |     201.3 MB     |   **67.1 MB**    |

## Conclusion

For the tested sequence lengths, `phastft` consistently outperformed `rustfft` in terms of memory usage and time taken.

However, `phastft` has several limitations:
- requires nightly Rust due to usage of [portable SIMD](https://doc.rust-lang.org/1.76.0/std/simd/index.html)
- does not support computing Fourier transforms for sequences with lengths that are not a power of 2; a workaround for this is to pad the sequence as necessary
- does not support `f32` for computing Fourier transforms, unlike `rustfft` which supports both `f32` and `f64`

If these don't matter for your use case, `phastft` is an excellent choice.