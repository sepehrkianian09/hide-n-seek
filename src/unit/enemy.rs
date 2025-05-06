#[derive(Default)]
pub struct Enemy {
    speed: f64,
}

impl Enemy {
    pub fn with_speed(speed: f64) -> Self {
        Self { speed: speed }
    }
}