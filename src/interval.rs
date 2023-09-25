#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn surrounds(self, x: f32) -> bool {
        x > self.min && x < self.max
    }
}
