//! PRNG based on https://www.coranac.com/tonc/text/gfx.htm

pub struct Rng {
    state: u32,
}

impl Rng {
    // Creates an `Rng` object with a given seed.
    pub fn new(seed: u32) -> Rng {
        Rng { state: seed }
    }

    // Returns a random integer in range `[min, max)`.
    pub fn randint(&mut self, min: u32, max: u32) -> u32 {
        (self.next().unwrap() * (max - min)) >> 15 + min
    }
}

impl Iterator for Rng {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.state = 1664525 * self.state + 1013904223;
        Some((self.state >> 16) & 0x7FFF)
    }
}
