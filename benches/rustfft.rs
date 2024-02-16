use divan::{black_box, AllocProfiler, Bencher};
use fft_benchmark::{Float, LENGTHS, SEED};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn generate_numbers(count: usize) -> Vec<Complex<Float>> {
    let mut rng = Xoshiro256Plus::seed_from_u64(SEED);

    let mut nums = Vec::with_capacity(count);

    for _ in 0..count {
        nums.push(Complex::new(rng.gen(), rng.gen()));
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
        .bench_refs(|(planner, nums)| {
            planner.plan_fft_forward(nums.len()).process(nums);
            black_box(nums);
        });
}

#[divan::bench(args = LENGTHS)]
fn inverse(bencher: Bencher<'_, '_>, len: usize) {
    bencher
        .with_inputs(|| {
            let planner = FftPlanner::new();
            let nums = generate_numbers(len);
            (planner, nums)
        })
        .counter(len)
        .bench_refs(|(planner, nums)| {
            planner.plan_fft_inverse(nums.len()).process(nums);
            black_box(nums);
        });
}
