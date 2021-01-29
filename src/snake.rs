use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}
