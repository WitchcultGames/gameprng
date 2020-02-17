use super::Randomizable;
use std::u64;

pub trait PrngAlgorithm {
    fn next(&mut self) -> u64;

    fn random_factor(&mut self) -> f32 {
        self.next() as f32 / u64::MAX as f32
    }

    fn chance(&mut self, chance: f32) -> bool {
        chance >= self.random_factor()
    }
}

pub trait PrngSeeding {
    fn new(seed: u64) -> Self;
    fn seed(&mut self, seed: u64);
}

pub trait PrngGeneration {
    fn generate<T: Randomizable>(&mut self) -> T;
    fn range<T: Randomizable>(&mut self, minimum: T, maximum: T) -> T;
}
