use divan::{AllocProfiler, Bencher};
use fft_benchmark::{LENGTHS, SEED};
use phastft::{
    options::Options,
    planner::{Direction, Planner32, Planner64},
};
use rand::{
    distributions::{Distribution, Standard},
    Rng, SeedableRng,
};
use rand_xoshiro::Xoshiro256Plus;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn generate_numbers<T>(count: usize) -> (Vec<T>, Vec<T>)
where
    Standard: Distribution<T>,
{
    let mut rng = Xoshiro256Plus::seed_from_u64(SEED);

    let mut reals = Vec::with_capacity(count);
    let mut imags = Vec::with_capacity(count);

    for _ in 0..count {
        reals.push(rng.gen());
        imags.push(rng.gen());
    }

    (reals, imags)
}

struct Phastft;

trait Fft<T> {
    fn forward(reals: &mut [T], imags: &mut [T]);
    fn inverse(reals: &mut [T], imags: &mut [T]);
}

impl Fft<f32> for Phastft {
    fn forward(reals: &mut [f32], imags: &mut [f32]) {
        phastft::fft_32_with_opts_and_plan(
            reals,
            imags,
            &Options::default(),
            &mut Planner32::new(reals.len(), Direction::Forward),
        )
    }

    fn inverse(reals: &mut [f32], imags: &mut [f32]) {
        phastft::fft_32_with_opts_and_plan(
            reals,
            imags,
            &Options::default(),
            &mut Planner32::new(reals.len(), Direction::Reverse),
        )
    }
}

impl Fft<f64> for Phastft {
    fn forward(reals: &mut [f64], imags: &mut [f64]) {
        phastft::fft_64_with_opts_and_plan(
            reals,
            imags,
            &Options::default(),
            &mut Planner64::new(reals.len(), Direction::Forward),
        )
    }

    fn inverse(reals: &mut [f64], imags: &mut [f64]) {
        phastft::fft_64_with_opts_and_plan(
            reals,
            imags,
            &Options::default(),
            &mut Planner64::new(reals.len(), Direction::Reverse),
        )
    }
}

#[divan::bench(types = [f32, f64], args = LENGTHS)]
fn forward<T>(bencher: Bencher<'_, '_>, len: usize)
where
    Standard: Distribution<T>,
    Phastft: Fft<T>,
{
    bencher
        .with_inputs(|| generate_numbers(len))
        .counter(len)
        .bench_refs(|(reals, imags)| Phastft::forward(reals, imags));
}

#[divan::bench(types = [f32, f64], args = LENGTHS)]
fn inverse<T>(bencher: Bencher<'_, '_>, len: usize)
where
    Standard: Distribution<T>,
    Phastft: Fft<T>,
{
    bencher
        .with_inputs(|| generate_numbers(len))
        .counter(len)
        .bench_refs(|(reals, imags)| Phastft::inverse(reals, imags));
}
