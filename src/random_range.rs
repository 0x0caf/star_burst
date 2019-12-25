use rand::{thread_rng, Rng};
#[derive(Clone, Copy)]
pub struct RandomRange {
    pub min: f32,
    pub max: f32,
}

impl RandomRange {
    pub fn new(min: f32, max: f32) -> RandomRange {
        RandomRange {
            min, max
        }
    }

    pub fn get(&self) -> f32 {
        let mut rng = thread_rng();
        rng.gen_range(self.min, self.max)
    }

    pub fn bool(&self) -> bool {
        let mut rng = thread_rng();
        rng.gen_bool(self.min as f64 / self.max as f64)
    }

    pub fn positive_or_negative(&self) -> f32 {
        if self.bool() {
            1.0
        } else {
            -1.0
        }
    }
}