use crate::entity::tile::Tile;

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

pub struct Action {
    pub tile: Tile,
    pub direction: Direction,
    pub op: Operation,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.tile.col,
            self.tile.row,
            self.direction.to_string(),
            self.op.to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_to_string() {
        let tile = Tile {
            value: 5,
            row: 2,
            col: 3,
        };
        let action = Action {
            tile,
            direction: Direction::Up,
            op: Operation::Plus,
        };
        assert_eq!(action.to_string(), String::from("3 2 U +"));
    }
}
