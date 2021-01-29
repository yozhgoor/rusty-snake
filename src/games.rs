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
