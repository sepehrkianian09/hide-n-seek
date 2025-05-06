use std::{cmp::min, f64::consts::PI, fmt::Display};

use crate::{point::Point2d, traits::Position, ui::draw::Draw};

#[derive(Default)]
pub struct Player {
    speed: f64,
    health: u8,
    pub position: Point2d<f64>,
    direction: Point2d<f64>,
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

impl Player {
    fn change_speed(&mut self, change: f64) {
        self.speed += change;
        self.speed = self.speed.max(0.0).min(1.0);
    }

    pub fn accelerate(&mut self) {
        self.change_speed(0.05);
    }

    pub fn decelerate(&mut self) {
        self.change_speed(-0.05);
    }

    pub fn move_forward(&mut self) {
        self.position += self.direction.clone() * self.speed;
    }

    pub fn forward_position(&self) -> Point2d<u16> {
        (self.position.clone() + self.direction.clone() * self.speed).to_u16()
    }

    fn rotate(pos: &mut Point2d<f64>, angle_rad: f64) {
        let cos_theta = angle_rad.cos();
        let sin_theta = angle_rad.sin();
        
        pos.x = pos.x * cos_theta - pos.y * sin_theta;
        pos.y = pos.x * sin_theta + pos.y * cos_theta;
    }

    pub fn turn_left(&mut self) {
        Self::rotate(&mut self.direction, -PI / 4.0);
    }

    pub fn turn_right(&mut self) {
        Self::rotate(&mut self.direction, PI / 4.0);
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
    direction: Point2d<f64>,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self {
            speed: 0.0,
            health: 0,
            direction: Point2d { x: 1.0, y: 0.0 }
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

    pub fn direction(mut self, x: f64, y: f64) -> Self {
        self.direction = Point2d { x, y };
        self
    }

    pub fn build(&self) -> Player {
        Player {
            speed: self.speed,
            health: self.health,
            position: Point2d::default(),
            direction: self.direction.clone(),
        }
    }
}
