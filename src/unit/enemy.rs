use crate::{point::Point2d, traits::Position};

#[derive(Default)]
pub struct Enemy {
    speed: f64,
    pub position: Point2d<f64>,
}

impl Enemy {
    pub fn with_speed(speed: f64) -> Self {
        Self { speed: speed, position: Point2d::default() }
    }
}

impl Position<f64> for Enemy {
    fn position(&self) -> Point2d<f64> {
        self.position.clone()
    }

    fn set_position(&mut self, position: Point2d<f64>) {
        self.position = position;
    }
}