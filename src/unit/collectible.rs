use std::fmt::Display;

use crate::{point::Point2d, traits::Position, ui::draw::Draw};

#[derive(Default)]
pub struct Collectible {
    pub position: Point2d<u16>,
}

impl Position<u16> for Collectible {
    fn position(&self) -> Point2d<u16> {
        self.position.clone()
    }

    fn set_position(&mut self, position: Point2d<u16>) {
        self.position = position;
    }
}

impl Display for Collectible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bright_yellow = "\x1b[93m"; // Bright yellow
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";
        
        write!(f, "{}{}✨{}", bold, bright_yellow, reset)
    }
}

impl Draw<u16> for Collectible {}