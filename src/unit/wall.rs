use std::fmt::Display;

use crate::{point::Point2d, traits::Position, ui::draw::Draw};

#[derive(Default)]
pub struct Wall {
    position: Point2d<u16>,
}

impl Wall {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            position: Point2d::new(x, y),
        }
    }
}

impl Position<u16> for Wall {
    fn position(&self) -> Point2d<u16> {
        self.position.clone()
    }

    fn set_position(&mut self, position: Point2d<u16>) {
        self.position = position;
    }
}

impl Display for Wall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let red = "\x1b[31m";
        let rectangle = "█";
        let reset = "\x1b[0m";

        write!(f, "{}{}{}", red, rectangle, reset)
    }
}

impl Draw<u16> for Wall {}
