use crate::{point::Point2d, traits::Position};

use crate::unit::Player;

pub struct Hud<'a> {
    score: u32,
    player: &'a Player,
    position: Point2d<u16>,
}

impl<'a> Hud<'a> {
    pub fn new(score: u32, player: &'a Player, position: Point2d<u16>) -> Self {
        Self { score, player, position }
    }

    pub fn text(&self) -> String {
        format!("Health: {}, Score: {}", self.player.health(), self.score)
    }
}

impl<'a> Position<u16> for Hud<'a> {
    fn position(&self) -> crate::point::Point2d<u16> {
        self.position.clone()
    }

    fn set_position(&mut self, position: crate::point::Point2d<u16>) {
        self.position = position;
    }
}