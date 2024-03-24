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

pub struct Action {
    pub row: usize,
    pub col: usize,
    pub direction: Direction,
    pub op: Operation,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.col,
            self.row,
            self.direction.to_string(),
            self.op.to_string()
        )
    }
}

impl Clone for Action {
    fn clone(&self) -> Self {
        Action {
            row: self.row,
            col: self.col,
            direction: self.direction.clone(),
            op: self.op.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_to_string() {
        let action = Action {
            row: 2,
            col: 3,
            direction: Direction::Up,
            op: Operation::Plus,
        };
        assert_eq!(action.to_string(), String::from("3 2 U +"));
    }
}
