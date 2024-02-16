// PhastFT only supports 64-bit floating point numbers for computing Fourier transforms
pub type Float = f64;

pub const SEED: u64 = 42;

pub const LENGTHS: [usize; 19] = [
    8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131_072, 262_144,
    524_288, 1_048_576, 2_097_152,
];
