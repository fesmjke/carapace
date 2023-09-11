pub struct LCG {
    modulus: i32,
    multiplier: i32,
    increment: i32,
    seed: i32,
}

impl LCG {
    pub fn new(modulus: i32, multiplier: i32, increment: i32, seed: i32) -> Self {
        Self {
            modulus,
            multiplier,
            increment,
            seed,
        }
    }
}

impl Iterator for LCG {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed = (self.multiplier * self.seed + self.increment) % self.modulus;

        Some(self.seed)
    }
}
