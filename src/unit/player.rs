use std::{
    f64::consts::PI,
    fmt::{Display, Formatter},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::{
    game::Game,
    point::Point2d,
    traits::{Position, UpdatableByTimeFrame},
};

pub mod builder;
pub use builder::PlayerBuilder;

pub mod state;
pub use state::PlayerState;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Player {
    position: Point2d<f64>,
    direction: Point2d<f64>,
    speed: f64,
}

impl Player {
    pub fn new(position: Point2d<f64>, direction: Point2d<f64>, speed: f64) -> Self {
        Self {
            position,
            direction,
            speed,
        }
    }

    pub fn builder() -> PlayerBuilder {
        PlayerBuilder::new()
    }

    pub fn turn_left(&mut self) {
        let angle = PI / 4.0;
        self.direction = self.direction.rotate(angle);
    }

    pub fn turn_right(&mut self) {
        let angle = -PI / 4.0;
        self.direction = self.direction.rotate(angle);
    }

    pub fn accelerate(&mut self) {
        self.speed += 0.1;
        self.speed = self.speed.min(1.0);
    }

    pub fn decelerate(&mut self) {
        self.speed -= 0.1;
        self.speed = self.speed.max(0.0);
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crossterm::style::Stylize;

        let icon = if self.speed == 0.0 {
            "•"
        } else {
            match self.direction.round().to_i16() {
                Point2d { x: 0, y: -1 } => "↑",
                Point2d { x: 1, y: -1 } => "↗",
                Point2d { x: 1, y: 0 } => "→",
                Point2d { x: 1, y: 1 } => "↘",
                Point2d { x: 0, y: 1 } => "↓",
                Point2d { x: -1, y: 1 } => "↙",
                Point2d { x: -1, y: 0 } => "←",
                Point2d { x: -1, y: -1 } => "↖",
                _ => "•",
            }
        };
        write!(f, "{}", icon.dark_blue())
    }
}

impl Player {
    fn forward_position(&self, since_last_time: &Duration) -> Point2d<f64> {
        self.position + (self.direction * (self.speed * since_last_time.as_secs_f64()))
    }
}

impl UpdatableByTimeFrame for Player {
    fn update(&mut self, game: &Game) {
        // move player if not colliding with a wall
        let player_next_position = self.forward_position(&game.update_interval_millis);
        if !game.do_walls_collide(player_next_position.round().to_u16()) {
            self.position = player_next_position;
        }
    }
}
