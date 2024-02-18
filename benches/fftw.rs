use core::ptr::slice_from_raw_parts_mut;
use divan::Bencher;
use fft_benchmark::{Float, LENGTHS, SEED};
use fftw::array::AlignedVec;
use fftw::plan::{C2CPlan, C2CPlan64};
use fftw::types::{Flag, Sign};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use rustfft::num_complex::Complex;

fn main() {
    divan::main();
}

fn generate_numbers(count: usize) -> AlignedVec<Complex<Float>> {
    let mut rng = Xoshiro256Plus::seed_from_u64(SEED);

    let mut nums = AlignedVec::new(count);

    for num in nums.iter_mut() {
        *num = Complex::new(rng.gen(), rng.gen())
    }

    nums
}

#[divan::bench(args = LENGTHS)]
fn forward(bencher: Bencher<'_, '_>, len: usize) {
    bencher
        .with_inputs(|| generate_numbers(len))
        .counter(len)
        .bench_refs(|nums| {
            C2CPlan64::aligned(&[len], Sign::Forward, Flag::DESTROYINPUT | Flag::ESTIMATE)
                .unwrap()
                .c2c(
                    // SAFETY: This is *probably* fine. The FFT needs to be computed in-place to match the behavior
                    // of other libraries included in the benchmark. To do this, we need to have two mutable borrows
                    // of the same slice when calling `fftw::plan::C2CPlan::c2c()`. This seems to be fine
                    // since there is existing C code using FFTW that does this; see
                    // https://github.com/QuState/PhastFT/blob/77e74fb8db0f83e57026727dd5b6b5fb287ef802/benches/main.c#L84.
                    unsafe { &mut *slice_from_raw_parts_mut(nums.as_mut_ptr(), len) },
                    nums,
                )
                .unwrap()
        });
}

#[divan::bench(args = LENGTHS)]
fn inverse(bencher: Bencher<'_, '_>, len: usize) {
    bencher
        .with_inputs(|| generate_numbers(len))
        .counter(len)
        .bench_refs(|nums| {
            C2CPlan64::aligned(&[len], Sign::Backward, Flag::DESTROYINPUT | Flag::ESTIMATE)
                .unwrap()
                .c2c(
                    // SAFETY: See above comment.
                    unsafe { &mut *slice_from_raw_parts_mut(nums.as_mut_ptr(), len) },
                    nums,
                )
                .unwrap()
        });
}
