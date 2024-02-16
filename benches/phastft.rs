use divan::{black_box, AllocProfiler, Bencher};
use phastft::planner::Direction;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

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
        .bench_values(|(mut reals, mut imags)| {
            phastft::fft(&mut reals, &mut imags, Direction::Forward);
            black_box((reals, imags));
        });
}

#[divan::bench(args = LENGTHS)]
fn inverse(bencher: Bencher<'_, '_>, len: usize) {
    bencher
        .with_inputs(|| generate_numbers(len))
        .counter(len)
        .bench_values(|(mut reals, mut imags)| {
            phastft::fft(&mut reals, &mut imags, Direction::Reverse);
            black_box((reals, imags));
        });
}
