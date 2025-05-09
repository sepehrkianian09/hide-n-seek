use num::traits::NumAssign;
use rand::{
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    Rng, RngCore,
};
use std::{ops::Range, time::Duration};

use crate::{game::Game, point::Point2d};

pub trait Position<T: NumAssign + Copy> {
    fn position(&self) -> Point2d<T>;
    fn set_position(&mut self, position: Point2d<T>);
    fn set_rand_position(&mut self, rng: &mut Box<dyn RngCore>, x_range: Range<T>, y_range: Range<T>)
    where
        T: PartialOrd + SampleUniform,
        Standard: Distribution<T>,
    {
        let new_position = Point2d::new(rng.gen_range(x_range), rng.gen_range(y_range));
        self.set_position(new_position);
    }
}


pub trait UpdatableByTimeFrame {
    fn update(&mut self, game: &Game, since_last_time: &Duration);
}