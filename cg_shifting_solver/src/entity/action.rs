use crate::entity::enums::{Direction, Operation};

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

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
            && self.col == other.col
            && self.direction == other.direction
            && self.op == other.op
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
