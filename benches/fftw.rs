use core::ptr::slice_from_raw_parts_mut;
use divan::Bencher;
use fft_benchmark::{LENGTHS, SEED};
use fftw::{
    array::{AlignedAllocable, AlignedVec},
    plan::{C2CPlan, C2CPlan32, C2CPlan64},
    types::{Flag, Sign},
};
use rand::{
    distributions::{Distribution, Standard},
    Rng, SeedableRng,
};
use rand_xoshiro::Xoshiro256Plus;
use rustfft::num_complex::Complex;

fn main() {
    divan::main();
}

fn generate_numbers<T>(count: usize) -> AlignedVec<Complex<T>>
where
    Complex<T>: AlignedAllocable,
    Standard: Distribution<T>,
{
    let mut rng = Xoshiro256Plus::seed_from_u64(SEED);

    let mut nums = AlignedVec::new(count);

    for num in nums.iter_mut() {
        *num = Complex::new(rng.gen(), rng.gen())
    }

    nums
}

struct Fftw;

trait Planner<T> {
    type Plan: C2CPlan<Complex = Complex<T>>;
}

impl Planner<f32> for Fftw {
    type Plan = C2CPlan32;
}

impl Planner<f64> for Fftw {
    type Plan = C2CPlan64;
}

#[divan::bench(types = [f32, f64], args = LENGTHS)]
fn forward<T>(bencher: Bencher<'_, '_>, len: usize)
where
    Complex<T>: AlignedAllocable,
    Standard: Distribution<T>,
    Fftw: Planner<T>,
{
    bencher
        .with_inputs(|| generate_numbers(len))
        .counter(len)
        .bench_refs(|nums| {
            <Fftw as Planner<T>>::Plan::aligned(
                &[len],
                Sign::Forward,
                Flag::DESTROYINPUT | Flag::ESTIMATE,
            )
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

#[divan::bench(types = [f32, f64], args = LENGTHS)]
fn inverse<T>(bencher: Bencher<'_, '_>, len: usize)
where
    Complex<T>: AlignedAllocable,
    Standard: Distribution<T>,
    Fftw: Planner<T>,
{
    bencher
        .with_inputs(|| generate_numbers(len))
        .counter(len)
        .bench_refs(|nums| {
            <Fftw as Planner<T>>::Plan::aligned(
                &[len],
                Sign::Backward,
                Flag::DESTROYINPUT | Flag::ESTIMATE,
            )
            .unwrap()
            .c2c(
                // SAFETY: See above comment.
                unsafe { &mut *slice_from_raw_parts_mut(nums.as_mut_ptr(), len) },
                nums,
            )
            .unwrap()
        });
}
