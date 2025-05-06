use std::fmt::{format, Display};

use crate::ui::draw::Draw;
use crate::{point::Point2d, traits::Position};

use crate::unit::Player;

pub struct Hud<'a> {
    score: u32,
    player: &'a Player,
    y_position: u16,
}

impl<'a> Hud<'a> {
    pub fn new(score: u32, player: &'a Player, y_position: u16) -> Self {
        Self { score: score, player: player, y_position: y_position }
    }

    pub fn text(&self) -> String {
        format!("Player: {}\nScore: {}", self.player.to_string(), self.score)
    }
}

impl<'a> Position<u16> for Hud<'a> {
    fn position(&self) -> crate::point::Point2d<u16> {
        Point2d { x: 0, y: self.y_position }
    }

    fn set_position(&mut self, position: crate::point::Point2d<u16>) {
        self.y_position = position.y;
    }
}