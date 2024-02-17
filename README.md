# fft-benchmark

This benchmark compares the performance of two fast Fourier transform libraries for Rust: [`rustfft`](https://crates.io/crates/rustfft) and [`phastft`](https://crates.io/crates/phastft).

## Results

All measurements were taken on an M1 MacBook Pro with `rustc 1.78.0-nightly (8ace7ea1f 2024-02-07)`. For time taken, the first number is for the forward FFT, and the second number is for the inverse FFT. Median times are listed. For each row, the fastest time and the least total memory allocated are highlighted in **bold**.

| Sequence length |       `rustfft` time        |       `phastft` time        | `rustfft` memory | `phastft` memory |
| :-------------: | :-------------------------: | :-------------------------: | :--------------: | :--------------: |
|        8        |     165.6 ns / 125.9 ns     | **110.3 ns** / **103.2 ns** |      320 B       |     **64 B**     |
|       16        | **145.5 ns** / **137.7 ns** |     231.4 ns / 224.9 ns     |      480 B       |    **192 B**     |
|       32        | **170.2 ns** / **163.7 ns** |     424.1 ns / 413.7 ns     |      816 B       |    **448 B**     |
|       64        |   **666.6 ns** / 655.9 ns   |   681.9 ns / **645.4 ns**   |     3.968 KB     |    **960 B**     |
|       128       |     1.239 μs / 1.145 μs     | **1.051 μs** / **999.6 ns** |     7.04 KB      |   **1.984 KB**   |
|       256       |     2.291 μs / 2.29 μs      | **1.728 μs** / **1.728 μs** |     13.18 KB     |   **4.032 KB**   |
|       512       |     4.582 μs / 4.333 μs     | **3.124 μs** / **3.041 μs** |     25.47 KB     |   **8.128 KB**   |
|      1024       |     9.124 μs / 8.582 μs     | **5.749 μs** / **5.749 μs** |     50.04 KB     |   **16.32 KB**   |
|      2048       |     18.95 μs / 18.29 μs     | **11.83 μs** / **11.41 μs** |     99.2 KB      |   **32.7 KB**    |
|      4096       |     36.95 μs / 39.66 μs     | **24.12 μs** / **23.29 μs** |     197.5 KB     |   **65.47 KB**   |
|      8192       |     80.29 μs / 80.87 μs     | **53.04 μs** / **52.12 μs** |     394.1 KB     |    **131 KB**    |
|      16384      |      162 μs / 175.6 μs      | **123.3 μs** / **121.3 μs** |     787.3 KB     |    **262 KB**    |
|      32768      |     346.8 μs / 372.2 μs     | **345.4 μs** / **351.4 μs** |     1.573 MB     |   **524.2 KB**   |
|      65536      |    742.3 μs / **726 μs**    |   **731.8 μs** / 823.7 μs   |     3.146 MB     |   **1.048 MB**   |
|     131072      |     1.62 ms / 1.645 ms      | **1.591 ms** / **1.636 ms** |     6.292 MB     |   **2.097 MB**   |
|     262144      |   3.503 ms / **3.293 ms**   |   **3.475 ms** / 3.423 ms   |     12.58 MB     |   **4.194 MB**   |
|     524288      |     9.637 ms / 8.933 ms     | **7.601 ms** / **7.342 ms** |     25.16 MB     |   **8.388 MB**   |
|     1048576     |     16.3 ms / 16.26 ms      | **15.83 ms** / **15.77 ms** |     50.33 MB     |   **16.77 MB**   |
|     2097152     | **34.23 ms** / **33.99 ms** |      38.36 ms / 39 ms       |     100.6 MB     |   **33.55 MB**   |
|     4194304     |     72.24 ms / 72.01 ms     | **66.78 ms** / **66.85 ms** |     201.3 MB     |   **67.1 MB**    |
|     8388608     |     144.3 ms / 144.2 ms     | **139.7 ms** / **140.2 ms** |     402.6 MB     |   **134.2 MB**   |

## Conclusion

For the tested sequence lengths, `phastft` consistently outperformed `rustfft` in terms of memory usage and time taken.

However, `phastft` has several limitations:
- requires nightly Rust due to usage of [portable SIMD](https://doc.rust-lang.org/1.76.0/std/simd/index.html)
- does not support computing Fourier transforms for sequences with lengths that are not a power of 2; a workaround for this is to pad the sequence as necessary
- does not support `f32` for computing Fourier transforms, unlike `rustfft` which supports both `f32` and `f64`

If these don't matter for your use case, `phastft` is an excellent choice.