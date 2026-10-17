//! Prosty generator LCG — wystarczy do iteracji 1; bez zewnętrznych zależności.

#[derive(Debug, Clone)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        // Unikaj zera — LCG wtedy utyka.
        let state = if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        };
        Self { state }
    }

    pub fn from_state(state: u64) -> Self {
        Self::new(state)
    }

    pub fn state(&self) -> u64 {
        self.state
    }

    pub fn next_u64(&mut self) -> u64 {
        // Numerical Recipes LCG
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    pub fn gen_range(&mut self, min_inclusive: u32, max_inclusive: u32) -> u32 {
        debug_assert!(min_inclusive <= max_inclusive);
        let span = u64::from(max_inclusive - min_inclusive) + 1;
        min_inclusive + (self.next_u64() % span) as u32
    }

    pub fn chance(&mut self, numerator: u32, denominator: u32) -> bool {
        debug_assert!(denominator > 0);
        self.gen_range(1, denominator) <= numerator
    }

    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            return None;
        }
        let idx = self.gen_range(0, (items.len() - 1) as u32) as usize;
        items.get(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..20 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }
}
