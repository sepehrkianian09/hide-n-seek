use serde::{Deserialize, Serialize};

use crate::traits::UpdatableByTimeFrame;
use crate::{point::Point2d, traits::Position};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hud {
    score: u32,
    health: u8,
    position: Point2d<u16>,
}

impl Hud {
    pub fn new(position: Point2d<u16>) -> Self {
        Self {
            score: 0,
            health: 0,
            position,
        }
    }

    pub fn text(&self) -> String {
        format!("Health: {}, Score: {}", self.health, self.score)
    }

    pub fn set(&mut self, score: u32, health: u8) {
        self.score = score;
        self.health = health;
    }
}

impl Position<u16> for Hud {
    fn position(&self) -> crate::point::Point2d<u16> {
        self.position.clone()
    }

    fn set_position(&mut self, position: crate::point::Point2d<u16>) {
        self.position = position;
    }
}

impl UpdatableByTimeFrame for Hud {
    fn update(&mut self, game: &crate::game::Game) {
        self.set(game.player_score(), game.player_health());
    }
}
