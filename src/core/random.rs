use godot::register::GodotClass;

const A: u64 = 6364136223846793005;
const C: u64 = 1;
const M: u64 = u64::MAX;

#[derive(GodotClass)]
#[class(init)]
pub struct LinearCongruentialGenerator {
    state: u64,
    seed: u64,
}

impl LinearCongruentialGenerator {
    pub fn new(seed: u64) -> Self {
        Self { state: seed, seed }
    }
    pub fn gen(&mut self) -> u64 {
        self.state = u64::wrapping_add(u64::wrapping_mul(A, self.state), C) % M;
        self.state
    }

    pub fn set_seed(&mut self, seed: i64) {
        self.seed = seed as u64;
        self.state = seed as u64;
    }
}

impl Default for LinearCongruentialGenerator {
    fn default() -> Self {
        Self::new(0)
    }
}
