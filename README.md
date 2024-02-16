# fft-benchmark

This benchmark compares the performance of two Fast Fourier transform libraries for Rust: [`rustfft`](https://crates.io/crates/rustfft) and [`phastft`](https://crates.io/crates/phastft).

## Results

All measurements were taken on an M1 MacBook Pro with `rustc 1.78.0-nightly (8ace7ea1f 2024-02-07)`. For time taken, the first number is for the forward FFT, and the second number is for the inverse FFT. Median times are listed. For memory usage, the first number is bytes allocated in total, while the second number is bytes deallocated in total. For each row, the fastest time and the least total memory allocated/deallocated are highlighted in **bold**.

| Sequence length |       `rustfft` time        |       `phastft` time        |  `rustfft` memory   |    `phastft` memory     |
| :-------------: | :-------------------------: | :-------------------------: | :-----------------: | :---------------------: |
|        8        |     249.6 ns / 290.6 ns     | **201.4 ns** / **191.7 ns** |    320 B / 448 B    |    **64 B / 192 B**     |
|       16        |  **260 ns** / **249.6 ns**  |     343.4 ns / 327.7 ns     |    480 B / 736 B    |    **192 B / 448 B**    |
|       32        | **275.7 ns** / **286.1 ns** |     499.6 ns / 494.5 ns     |  816 B / 1.328 KB   |    **448 B / 960 B**    |
|       64        |     812.2 ns / 806.9 ns     | **786.1 ns** / **723.6 ns** | 3.968 KB / 3.712 KB |  **960 B / 1.984 KB**   |
|       128       |     1.332 μs / 1.312 μs     | **1.114 μs** / **1.083 μs** | 7.04 KB / 6.912 KB  | **1.984 KB / 4.032 KB** |
|       256       |     2.395 μs / 2.291 μs     | **1.853 μs** / **1.78 μs**  | 13.18 KB / 12.92 KB | **4.032 KB / 8.128 KB** |
|       512       |     4.583 μs / 4.458 μs     | **3.332 μs** / **3.145 μs** | 25.47 KB / 25.34 KB | **8.128 KB / 16.32 KB** |
|      1024       |     9.29 μs / 8.749 μs      | **6.166 μs** / **5.832 μs** | 50.04 KB / 49.79 KB | **16.32 KB / 32.7 KB**  |
|      2048       |     19.64 μs / 18.58 μs     | **12.24 μs** / **11.45 μs** | 99.2 KB / 99.07 KB  | **32.7 KB / 65.47 KB**  |
|      4096       |     37.22 μs / 42.31 μs     | **23.41 μs** / **23.83 μs** | 197.5 KB / 197.2 KB |  **65.47 KB / 131 KB**  |
|      8192       |     77.2 μs / 86.83 μs      | **53.66 μs** / **53.27 μs** | 394.1 KB / 393.9 KB |   **131 KB / 262 KB**   |
|      16384      |      165.5 μs / 190 μs      |  **120 μs** / **159.7 μs**  |  787.3 KB / 787 KB  |  **262 KB / 524.2 KB**  |
|      32768      |   373.7 μs / **372.6 μs**   |   **350.3 μs** / 397.7 μs   | 1.573 MB / 1.573 MB | **524.2 KB / 1.048 MB** |
|      65536      |    767.1 μs / **777 μs**    |   **729.4 μs** / 843.8 μs   | 3.146 MB / 3.146 MB | **1.048 MB / 2.097 MB** |
|     131072      |     1.783 ms / 1.747 ms     | **1.618 ms** / **1.662 ms** | 6.292 MB / 6.292 MB | **2.097 MB / 4.194 MB** |
|     262144      |     3.834 ms / 3.612 ms     | **3.498 ms** / **3.582 ms** | 12.58 MB / 12.58 MB | **4.194 MB / 8.388 MB** |
|     524288      |     10.78 ms / 10.14 ms     | **8.09 ms** / **7.933 ms**  | 25.16 MB / 25.16 MB | **8.388 MB / 16.77 MB** |
|     1048576     |     18.05 ms / 19.32 ms     |   **17.31 ms / 17.11 ms**   | 50.33 MB / 50.33 MB | **16.77 MB / 33.55 MB** |
|     2097152     |   **36.63 ms** / 40.81 ms   |   40.53 ms / **40.3 ms**    | 100.6 MB / 100.6 MB | **33.55 MB / 67.1 MB**  |
|     4194304     |     81.86 ms / 81.79 ms     | **70.52 ms** / **71.86 ms** | 201.3 MB / 201.3 MB | **67.1 MB / 134.2 MB**  |

## Conclusion

For the tested sequence lengths, `phastft` consistently outperformed `rustfft` in terms of memory usage and time taken.

However, `phastft` cannot calculate Fourier transforms for sequences with lengths that are not a power of 2. Instead, the library [panics](https://github.com/QuState/PhastFT/blob/a8d948561152d8dce760383b463d3b59cf897f0b/src/lib.rs#L68). If this doesn't apply to your use case, `phastft` is a good choice.