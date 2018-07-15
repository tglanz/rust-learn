#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Direction {
    pub fn are_oposites(a: Direction, b: Direction) -> bool {
        match a {
            Direction::Up => b == Direction::Down,
            Direction::Right => b == Direction::Left,
            Direction::Down => b == Direction::Up,
            Direction::Left => b == Direction::Right,
        }
    }
}