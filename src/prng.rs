use super::{PrngAlgorithm, PrngGeneration, PrngSeeding, Randomizable};

pub struct Prng<T: PrngAlgorithm>(T);

impl<G: PrngAlgorithm + PrngSeeding + PrngGeneration> Prng<G> {
    pub fn new(seed: u64) -> Prng<G> {
        Prng { 0: G::new(seed) }
    }

    pub fn random_factor(&mut self) -> f32 {
        self.0.random_factor()
    }

    pub fn chance(&mut self, chance: f32) -> bool {
        self.0.chance(chance)
    }

    pub fn generate<T: Randomizable>(&mut self) -> T {
        self.0.generate()
    }

    pub fn range<T: Randomizable>(&mut self, minimum: T, maximum: T) -> T {
        self.0.range(minimum, maximum)
    }
}
