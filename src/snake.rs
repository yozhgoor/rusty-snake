use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();
        let body: Vec<Point> = (0..length)
            .into_iter()
            .map(|i| start.transform(opposite, i))
            .collect();

        Self { body, direction, digesting: false }
    }
    pub fn get_head_point(&self) -> Point {
        self.body.first().unwrap().clone()
    }
    pub fn get_body_points(&self) -> Vec<Point> {
        self.body.clone()
    }
    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }
}
