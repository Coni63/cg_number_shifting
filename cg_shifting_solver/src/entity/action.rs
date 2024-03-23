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

pub struct Action<'a> {
    pub row: usize,
    pub col: usize,
    pub direction: &'a Direction,
    pub op: &'a Operation,
    pub source_value: u16,
    pub target_value: u16,
}

impl<'a> ToString for Action<'a> {
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

impl Clone for Action<'_> {
    fn clone(&self) -> Self {
        Action {
            row: self.row,
            col: self.col,
            direction: self.direction,
            op: self.op,
            source_value: self.source_value,
            target_value: self.target_value,
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
            direction: &Direction::Up,
            op: &Operation::Plus,
            source_value: 5,
            target_value: 10,
        };
        assert_eq!(action.to_string(), String::from("3 2 U +"));
    }
}
