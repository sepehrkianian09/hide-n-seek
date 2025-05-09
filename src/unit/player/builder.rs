use crate::point::Point2d;

use super::Player;

pub struct PlayerBuilder {
    position: Point2d<f64>,
    direction: Point2d<f64>,
    speed: f64,
    health: u8,
    score: u32,
}

#[allow(clippy::new_without_default)]
impl PlayerBuilder {
    pub fn new() -> Self {
        Self {
            position: Point2d::new(1.0, 1.0),
            direction: Point2d::new(1.0, 0.0),
            speed: 0.0,
            health: 10,
            score: 0,
        }
    }

    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Point2d::new(x, y);
        self
    }

    pub fn direction(mut self, x: f64, y: f64) -> Self {
        self.direction = Point2d::new(x, y);
        self
    }

    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    pub fn health(mut self, health: u8) -> Self {
        self.health = health;
        self
    }

    pub fn score(mut self, score: u32) -> Self {
        self.score = score;
        self
    }

    pub fn build(self) -> Player {
        Player {
            position: self.position,
            direction: self.direction,
            speed: self.speed,
            health: self.health,
            score: self.score,
        }
    }
}
