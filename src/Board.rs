use crate::Aox::{get_board_draw_positions, Vec2D};
use crate::Pice::Pice;

pub struct Board {
    pub BlackPices: Vec<Pice>,
    pub WhitePices: Vec<Pice>,
    pub positions: Vec<Vec<Vec2D>>,
}

impl Board {
    pub fn new() -> Board {
        let mut black_pices = Vec::new();
        let mut white_pices = Vec::new();
        let pos = get_board_draw_positions(0, 0, 1000);

        let back_rank_ids = [2, 4, 3, 1, 0, 3, 4, 2];

        for (col, &id) in back_rank_ids.iter().enumerate() {
            black_pices.push(Pice::new(Vec2D::new(0, col as i32), id, false));
        }
        for col in 0..8 {
            black_pices.push(Pice::new(Vec2D::new(1, col as i32), 5, false));
        }

        for (col, &id) in back_rank_ids.iter().enumerate() {
            white_pices.push(Pice::new(Vec2D::new(7, col as i32), id, true));
        }

        for col in 0..8 {
            white_pices.push(Pice::new(Vec2D::new(6, col as i32), 5, true));
        }

        Self {
            BlackPices: black_pices,
            WhitePices: white_pices,
            positions: pos,
        }
    }

    pub fn get_board_state(&self) -> Vec<Vec<i32>> {
        let mut board = vec![vec![-1; 8]; 8];

        for pice in &self.WhitePices {
            let r = pice.pos.x as usize;
            let c = pice.pos.y as usize;
            if r < 8 && c < 8 {
                board[r][c] = pice.TextureID;
            }
        }

        for pice in &self.BlackPices {
            let r = pice.pos.x as usize;
            let c = pice.pos.y as usize;
            if r < 8 && c < 8 {
                board[r][c] = pice.TextureID + 8;
            }
        }

        return board
    }
}