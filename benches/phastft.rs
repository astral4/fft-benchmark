use divan::{black_box, AllocProfiler, Bencher};
use fft_benchmark::{Float, LENGTHS, SEED};
use phastft::planner::Direction;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn generate_numbers(count: usize) -> (Vec<Float>, Vec<Float>) {
    let mut rng = Xoshiro256Plus::seed_from_u64(SEED);

    let mut reals = Vec::with_capacity(count);
    let mut imags = Vec::with_capacity(count);

    for _ in 0..count {
        reals.push(rng.gen());
        imags.push(rng.gen());
    }

    (reals, imags)
}

#[divan::bench(args = LENGTHS)]
fn forward(bencher: Bencher<'_, '_>, len: usize) {
    bencher
        .with_inputs(|| generate_numbers(len))
        .counter(len)
        .bench_refs(|(reals, imags)| {
            phastft::fft(reals, imags, Direction::Forward);
            black_box((reals, imags));
        });
}

#[divan::bench(args = LENGTHS)]
fn inverse(bencher: Bencher<'_, '_>, len: usize) {
    bencher
        .with_inputs(|| generate_numbers(len))
        .counter(len)
        .bench_refs(|(reals, imags)| {
            phastft::fft(reals, imags, Direction::Reverse);
            black_box((reals, imags));
        });
}
