pub enum WordSize {
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

impl WordSize {
    pub fn magic(&self) -> (u64, u64) {
        match self {
            WordSize::Sixteen => (0xB7E1, 0x9E37),
            WordSize::ThirtyTwo => (0xB7E15163, 0x9E3779B9),
            WordSize::SixtyFour => (0xB7E151628AED2A6B, 0x9E3779B97F4A7C15),
        }
    }
}

pub struct RC5 {
    // TODO
    word_size: WordSize,
    rounds: u8,
    octets: u8,
}

impl RC5 {
    pub fn new(word_size: WordSize, rounds: u8, octets: u8) -> Self {
        Self {
            word_size,
            rounds,
            octets,
        }
    }

    pub fn encrypt(&self, plain: &str, key: &str) -> String {
        todo!()
    }

    pub fn decrypt(&self) {
        todo!()
    }
}
