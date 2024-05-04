use divan::{AllocProfiler, Bencher};
use fft_benchmark::{LENGTHS, SEED};
use rand::{
    distributions::{Distribution, Standard},
    Rng, SeedableRng,
};
use rand_xoshiro::Xoshiro256Plus;
use rustfft::num_complex::Complex;
use rustfft::{FftNum, FftPlanner};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn generate_numbers<T>(count: usize) -> Vec<Complex<T>>
where
    Standard: Distribution<T>,
{
    let mut rng = Xoshiro256Plus::seed_from_u64(SEED);

    let mut nums = Vec::with_capacity(count);

    for _ in 0..count {
        nums.push(Complex::new(rng.gen(), rng.gen()));
    }

    nums
}

#[divan::bench(types = [f32, f64], args = LENGTHS)]
fn forward<T>(bencher: Bencher<'_, '_>, len: usize)
where
    T: FftNum,
    Standard: Distribution<T>,
{
    bencher
        .with_inputs(|| {
            let planner = FftPlanner::new();
            let nums = generate_numbers::<T>(len);
            (planner, nums)
        })
        .counter(len)
        .bench_refs(|(planner, nums)| planner.plan_fft_forward(nums.len()).process(nums));
}

#[divan::bench(types = [f32, f64], args = LENGTHS)]
fn inverse<T>(bencher: Bencher<'_, '_>, len: usize)
where
    T: FftNum,
    Standard: Distribution<T>,
{
    bencher
        .with_inputs(|| {
            let planner = FftPlanner::new();
            let nums = generate_numbers::<T>(len);
            (planner, nums)
        })
        .counter(len)
        .bench_refs(|(planner, nums)| planner.plan_fft_inverse(nums.len()).process(nums));
}
