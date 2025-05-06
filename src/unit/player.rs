use std::fmt::Display;

use crate::{point::Point2d, traits::Position, ui::draw::Draw};

#[derive(Default)]
pub struct Player {
    speed: f64,
    health: u8,
    pub position: Point2d<f64>,
}

impl Player {
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: u8) {
        if damage < self.health {
            self.health -= damage;
        } else {
            self.health = 0;
        }
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }
}

impl Position<f64> for Player {
    fn position(&self) -> Point2d<f64> {
        self.position.clone()
    }

    fn set_position(&mut self, position: Point2d<f64>) {
        self.position = position;
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let green = "\x1b[92m";
        let ufo = "\u{1F6F8}";
        let reset = "\x1b[0m";

        write!(f, "{}{}{}", green, ufo, reset)
    }
}

impl Draw<f64> for Player {}

pub struct PlayerBuilder {
    speed: f64,
    health: u8,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self {
            speed: 0.0,
            health: 0,
        }
    }

    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    pub fn health(mut self, health: u8) -> Self {
        self.health = health;
        self
    }

    pub fn build(&self) -> Player {
        Player {
            speed: self.speed,
            health: self.health,
            position: Point2d::default(),
        }
    }
}
