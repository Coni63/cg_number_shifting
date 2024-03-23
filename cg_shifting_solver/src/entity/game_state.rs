use crate::entity::tile::Tile;

pub struct GameState {
    pub tiles: Vec<Tile>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState { tiles: Vec::new() }
    }
}
