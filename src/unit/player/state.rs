use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PlayerState {
    health: u8,
    score: u32,
}

impl PlayerState {
    pub fn new(health: u8, score: u32,) -> Self {
        Self {
            health,
            score,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn decrease_health(&mut self) {
        self.health = self.health.saturating_sub(1);
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn increase_score(&mut self) {
        self.score += 1;
    }
}