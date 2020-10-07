use super::{PrngAlgorithm, PrngGeneration, PrngSeeding, Randomizable};
use std::num::Wrapping;

pub struct SplitMix64 {
    state: Wrapping<u64>,
}

impl PrngAlgorithm for SplitMix64 {
    fn next(&mut self) -> u64 {
        self.state += Wrapping::<u64>(0x9E37_79B9_7F4A_7C15);

        let mut z = self.state;

        z = (z ^ (z >> 30)) * Wrapping::<u64>(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)) * Wrapping::<u64>(0x94D0_49BB_1331_11EB);
        (z ^ (z >> 32)).0
    }
}

impl PrngSeeding for SplitMix64 {
    fn new(seed: u64) -> SplitMix64 {
        SplitMix64 {
            state: Wrapping(seed),
        }
    }

    fn seed(&mut self, seed: u64) {
        self.state = Wrapping(seed);
    }
}

impl PrngGeneration for SplitMix64 {
    fn generate<T: Randomizable>(&mut self) -> T {
        T::generate(self)
    }

    fn range<T: Randomizable>(&mut self, minimum: T, maximum: T) -> T {
        T::random_range(minimum, maximum, self)
    }
}

#[cfg(test)]
mod tests {
    use super::super::prng::Prng;
    use super::SplitMix64;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{f32, f64, i16, i32, i64, i8, u16, u32, u64, u8};

    #[test]
    fn random_factor() {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs() as u64,
            Err(_) => 666,
        };

        let mut prng = Prng::<SplitMix64>::new(seed);

        for _ in 0..1000000 {
            let f = prng.random_factor();

            assert!(f >= 0.0);
            assert!(f <= 1.0);
        }
    }

    #[test]
    fn chance() {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs() as u64,
            Err(_) => 666,
        };

        let mut prng = Prng::<SplitMix64>::new(seed);

        for _ in 0..1000000 {
            prng.chance(0.5);
        }
    }

    #[test]
    fn generate() {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs() as u64,
            Err(_) => 666,
        };

        let mut prng = Prng::<SplitMix64>::new(seed);

        for _ in 0..1000000 {
            let r = prng.generate::<i8>();

            assert!(r >= i8::MIN);
            assert!(r <= i8::MAX);
        }

        for _ in 0..1000000 {
            let r = prng.generate::<u8>();

            assert!(r >= u8::MIN);
            assert!(r <= u8::MAX);
        }

        for _ in 0..1000000 {
            let r = prng.generate::<i16>();

            assert!(r >= i16::MIN);
            assert!(r <= i16::MAX);
        }

        for _ in 0..1000000 {
            let r = prng.generate::<u16>();

            assert!(r >= u16::MIN);
            assert!(r <= u16::MAX);
        }

        for _ in 0..1000000 {
            let r = prng.generate::<i32>();

            assert!(r >= i32::MIN);
            assert!(r <= i32::MAX);
        }

        for _ in 0..1000000 {
            let r = prng.generate::<u32>();

            assert!(r >= u32::MIN);
            assert!(r <= u32::MAX);
        }

        for _ in 0..1000000 {
            let r = prng.generate::<i64>();

            assert!(r >= i64::MIN);
            assert!(r <= i64::MAX);
        }

        for _ in 0..1000000 {
            let r = prng.generate::<u64>();

            assert!(r >= u64::MIN);
            assert!(r <= u64::MAX);
        }

        for _ in 0..1000000 {
            let r = prng.generate::<f32>();

            assert!(r >= f32::MIN);
            assert!(r <= f32::MAX);
        }

        for _ in 0..1000000 {
            let r = prng.generate::<f64>();

            assert!(r >= f64::MIN);
            assert!(r <= f64::MAX);
        }
    }

    #[test]
    fn range() {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs() as u64,
            Err(_) => 666,
        };

        let mut prng = Prng::<SplitMix64>::new(seed);
        let mut bounds = (false, false);

        for a in 0..1000 {
            for b in 1..1000 {
                let max = a * b + b;
                let r = prng.range::<u64>(a, max);

                if r == a {
                    bounds.0 = true;
                }

                if r == max {
                    bounds.1 = true;
                }

                assert!(r >= a);
                assert!(r <= max);
            }
        }

        assert!(bounds.0 == true);
        assert!(bounds.1 == true);
    }
}
