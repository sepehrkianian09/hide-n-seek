use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{game::Game, point::Point2d, traits::Position};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Collectible {
    position: Point2d<u16>,
    score: u32,
}

impl Position<u16> for Collectible {
    fn position(&self) -> Point2d<u16> {
        self.position.clone()
    }

    fn set_position(&mut self, position: Point2d<u16>) {
        self.position = position;
    }
}

impl Collectible {
    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn randomize_position(&mut self, game: &Game) {
        game.randomize_position_u16(self);

        while game.do_walls_collide(self.position()) {
            game.randomize_position_u16(self);
        }
    }

    pub fn update(&mut self, game: &Game, since_last_time: &Duration) {
        let _ = since_last_time;
        // increase score if player collides with collectible
        if game.player_position().round().to_u16() == self.position() {
            self.score += 1;
            
            // move collectible to a new random position
            self.randomize_position(game);
        }
    }
}