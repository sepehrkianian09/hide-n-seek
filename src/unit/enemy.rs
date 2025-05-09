use std::time::Duration;

use crate::game::Game;
use crate::traits::UpdatableByTimeFrame;
use crate::{point::Point2d, traits::Position};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Enemy {
    position: Point2d<f64>,
    speed: f64,
}

impl Enemy {
    pub fn with_speed(speed: f64) -> Self {
        Self {
            position: Point2d::new(0.0, 0.0),
            speed,
        }
    }

    pub fn move_towards_player(
        &mut self,
        player_position: Point2d<f64>,
        since_last_time: &Duration,
    ) {
        let direction = player_position.round() - self.position().round();

        self.position += direction.normalize() * (self.speed * since_last_time.as_secs_f64());
    }
}

impl Position<f64> for Enemy {
    fn position(&self) -> Point2d<f64> {
        self.position
    }
    fn set_position(&mut self, position: Point2d<f64>) {
        self.position.x = position.x;
        self.position.y = position.y;
    }
}

impl UpdatableByTimeFrame for Enemy {
    fn update(&mut self, game: &Game, since_last_time: &Duration) {
        // move enemies
        self.move_towards_player(game.player_position(), &since_last_time);

        // reduce player health for each enemy collision
        if self.position().round() == game.player_position().round() {
            game.decrease_player_health();
        }
    }
}
