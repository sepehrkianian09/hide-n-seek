use crate::{
    traits::Position,
    unit::{Collectible, Enemy, Player, Wall},
};
use std::{fmt::Display, io::Write};

use crossterm::style::Stylize;
use num::{traits::NumAssign, NumCast};

// this is only an example, modify it to your needs or remove entirely
pub trait Draw<T: NumAssign + Copy + NumCast>: Position<T> + Display {
    fn draw(&self, stdout: &mut impl Write) {
        let position = self.position();

        crossterm::queue!(
            stdout,
            crossterm::cursor::MoveTo(
                position
                    .x
                    .to_f64()
                    .expect("could not convert position x to f64")
                    .round() as u16
                    + 1,
                position
                    .y
                    .to_f64()
                    .expect("could not convert position y to f64")
                    .round() as u16
                    + 1,
            ),
            crossterm::style::Print(self)
        )
        .unwrap();
    }
}
