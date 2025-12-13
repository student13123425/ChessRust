use crate::Aox::Vec2D;

#[derive(Debug, Clone)]
pub struct Move {
    pub start_sq: String,
    pub end_sq: String,
    pub piece_id: i32,
}

impl Move {
    pub fn new(start: String, end: String, piece_id: i32) -> Self {
        Self {
            start_sq: start,
            end_sq: end,
            piece_id,
        }
    }

    pub fn get_start_pos(&self) -> Vec2D {
        return self.parse_square(&self.start_sq)
    }

    pub fn get_end_pos(&self) -> Vec2D {
        return self.parse_square(&self.end_sq)
    }

    fn parse_square(&self, sq: &str) -> Vec2D {
        let bytes = sq.as_bytes();
        let col = (bytes[0] as char).to_ascii_lowercase() as i32 - 97;
        let row = 8 - ((bytes[1] as char) as i32 - 48);
        return Vec2D::new(row, col)
    }
}