use super::prng::Prng;
use super::splitmix64::SplitMix64;
use super::{PrngAlgorithm, PrngGeneration, PrngSeeding, Randomizable};
use std::num::Wrapping;

pub struct XorShift128Plus {
    state: [Wrapping<u64>; 2],
}

impl PrngAlgorithm for XorShift128Plus {
    fn next(&mut self) -> u64 {
        let x = self.state[0] ^ self.state[0] << 23;
        let y = self.state[1];
        self.state[0] = y;
        self.state[1] = x ^ y ^ (x >> 18) ^ (y >> 5);
        (self.state[1] + y).0
    }
}

impl PrngSeeding for XorShift128Plus {
    fn new(seed: u64) -> XorShift128Plus {
        let mut new = XorShift128Plus {
            state: [Wrapping(0), Wrapping(0)],
        };

        new.seed(seed);

        new
    }

    fn seed(&mut self, seed: u64) {
        let mut tmp_prng = Prng::<SplitMix64>::new(seed);

        self.state[0] = Wrapping(tmp_prng.generate());
        self.state[1] = Wrapping(tmp_prng.generate());
    }
}

impl PrngGeneration for XorShift128Plus {
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
    use super::XorShift128Plus;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{f32, f64, i16, i32, i64, i8, u16, u32, u64, u8};

    #[test]
    fn random_factor() {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs() as u64,
            Err(_) => 666,
        };

        let mut prng = Prng::<XorShift128Plus>::new(seed);

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

        let mut prng = Prng::<XorShift128Plus>::new(seed);

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

        let mut prng = Prng::<XorShift128Plus>::new(seed);

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

        let mut prng = Prng::<XorShift128Plus>::new(seed);

        for a in 0..1000 {
            for b in 1..1000 {
                let max = a * b + b;
                let r = prng.range::<u64>(a, max);

                assert!(r >= a);
                assert!(r <= max);
            }
        }
    }
}
