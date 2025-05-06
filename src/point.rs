use std::ops::{Add, AddAssign, Mul, Sub};

use num::{traits::NumAssign, ToPrimitive};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Point2d<T: NumAssign + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: NumAssign + Copy> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point2d<f64> {
    pub fn normalize(&self) -> Self {
        let Self { x, y } = self;
        let magnitude = (*x * *x + *y * *y).sqrt();
        Self { x: x / magnitude, y: y / magnitude }
    }
}

impl<T: NumAssign + Copy> Add for Point2d<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: NumAssign + Copy> Mul<T> for Point2d<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: NumAssign + Copy> AddAssign for Point2d<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: NumAssign + Copy> Sub for Point2d<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point2d<f64> {
    pub fn to_u16(&self) -> Point2d<u16> {
        Point2d {
            x: self.x.to_u16().unwrap(),
            y: self.y.to_u16().unwrap(),
        }
    }

    pub fn round(&self) -> Point2d<f64> {
        Point2d {
            x: self.x.round(),
            y: self.y.round(),
        }
    }
}
