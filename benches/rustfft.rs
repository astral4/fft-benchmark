use divan::{black_box, AllocProfiler, Bencher};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

type Float = f64;

const SEED: u64 = 42;

const LENGTHS: [usize; 19] = [
    8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131_072, 262_144,
    524_288, 1_048_576, 2_097_152,
];

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn generate_numbers(count: usize) -> Vec<Complex<Float>> {
    let mut rng = Xoshiro256Plus::seed_from_u64(SEED);

    let mut nums = Vec::with_capacity(count);

    for _ in 0..count {
        nums.push(Complex {
            re: rng.gen(),
            im: rng.gen(),
        });
    }

    nums
}

#[divan::bench(args = LENGTHS)]
fn forward(bencher: Bencher<'_, '_>, len: usize) {
    bencher
        .with_inputs(|| {
            let planner = FftPlanner::new();
            let nums = generate_numbers(len);
            (planner, nums)
        })
        .counter(len)
        .bench_values(|(mut planner, mut nums)| {
            planner.plan_fft_forward(nums.len()).process(&mut nums);
            black_box(nums);
        });
}
