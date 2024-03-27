#[derive(Debug, PartialEq)]
pub enum Metric {
    RemainingSum,
    RemainingTiles,
    ColRowsUsed,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => "U".to_string(),
            Direction::Down => "D".to_string(),
            Direction::Left => "L".to_string(),
            Direction::Right => "R".to_string(),
        }
    }
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Direction::Up => Direction::Up,
            Direction::Down => Direction::Down,
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Plus => "+".to_string(),
            Operation::Minus => "-".to_string(),
        }
    }
}

impl Clone for Operation {
    fn clone(&self) -> Self {
        match self {
            Operation::Plus => Operation::Plus,
            Operation::Minus => Operation::Minus,
        }
    }
}
