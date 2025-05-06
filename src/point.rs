use num::traits::NumAssign;

#[derive(Default, Clone, PartialEq)]
pub struct Point2d<T: NumAssign + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: NumAssign + Copy> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
