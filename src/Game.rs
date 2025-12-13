use crate::Aox::Vec2D;
use crate::Board::Board;

struct Game {
    board: Board,
    selected_pice:Vec2D
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            selected_pice:Vec2D::new(-1,-1)
        }
    }
}