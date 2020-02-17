pub mod prng;
pub mod prng_traits;
pub mod splitmix64;
pub mod xoroshiro128plus;
pub mod xorshift128plus;

use prng_traits::{PrngAlgorithm, PrngGeneration, PrngSeeding};

pub trait Randomizable {
    fn generate(algorithm: &mut PrngAlgorithm) -> Self;
    fn random_range(minimum: Self, maximum: Self, algorithm: &mut PrngAlgorithm) -> Self;
}

impl Randomizable for u8 {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        prng.next() as u8
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32 + 0.5;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for u16 {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        prng.next() as u16
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32 + 0.5;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for u32 {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        prng.next() as u32
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32 + 0.5;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for u64 {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        prng.next()
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32 + 0.5;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for usize {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        prng.next() as usize
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32 + 0.5;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for i8 {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        prng.next() as i8
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32 + 0.5;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for i16 {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        prng.next() as i16
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32 + 0.5;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for i32 {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        prng.next() as i32
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32 - 0.5;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for i64 {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        prng.next() as i64
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32 + 0.5;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for isize {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        prng.next() as isize
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32 + 0.5;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for f32 {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        std::f32::MAX * (prng.random_factor() - prng.random_factor())
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = (maximum - minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl Randomizable for f64 {
    fn generate(prng: &mut PrngAlgorithm) -> Self {
        std::f64::MAX * (prng.random_factor() - prng.random_factor()) as f64
    }

    fn random_range(minimum: Self, maximum: Self, prng: &mut PrngAlgorithm) -> Self {
        let diff = (maximum - minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}
