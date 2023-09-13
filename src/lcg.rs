pub struct LCG {
    modulus: u64,
    multiplier: u64,
    increment: u64,
    seed: u64,
}

impl LCG {
    pub fn new(modulus: u64, multiplier: u64, increment: u64, seed: u64) -> Self {
        Self {
            modulus,
            multiplier,
            increment,
            seed,
        }
    }
}

impl Iterator for LCG {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed = (self.multiplier * self.seed + self.increment) % self.modulus;

        Some(self.seed)
    }
}
