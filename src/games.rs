use crate::snake::Snake;
use crate::point::Point;
use crate::direction::Direction;
use std::io::Stdout;
use std::time{Duration, Instant};
use crossterm::terminal::size;
use crate::command::Command;
use rand::Rng;

const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;

#[derive(Debug)]
pub struct Game {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
    width: u16,
    height: u16,
    food: Option<Point>,
    snake: Snake,
    speed: u16,
    score: u16,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size: (u16, u16) = size.unwrap();
        Self{
            stdout,
            original_terminal_size,
            width,
            height,
            food: None,
            Snake: Snake::new(
                Point::new(width / 2, height / 2),
                3,
                match rand::thread_rng().gen_range(0, 4) {
                    0 => Direction::Up,
                    1 => Direction::Right,
                    2 => Direction::Down,
                    _ => Direction::Left,
                },
            ),
            speed: 0,
            score: 0,
        }
    }
}
