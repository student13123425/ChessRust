use crate::Aox::Vec2D;

#[derive(Debug, Clone)]
pub struct Move {
    pub start_sq: String,
    pub end_sq: String,
    pub piece_id: i32,
    pub side: bool,
}

impl Move {
    pub fn new(start: String, end: String, piece_id: i32, side: bool) -> Self {
        Self {
            start_sq: start,
            end_sq: end,
            piece_id,
            side,
        }
    }

    pub fn from_pos(start: Vec2D, end: Vec2D, piece_id: i32, side: bool) -> Self {
        let start_sq = Self::format_square(start);
        let end_sq = Self::format_square(end);
        Self {
            start_sq,
            end_sq,
            piece_id,
            side,
        }
    }

    pub fn get_start_pos(&self) -> Vec2D {
        self.parse_square(&self.start_sq)
    }

    pub fn get_end_pos(&self) -> Vec2D {
        self.parse_square(&self.end_sq)
    }

    fn parse_square(&self, sq: &str) -> Vec2D {
        let bytes = sq.as_bytes();
        let col = (bytes[0] as char).to_ascii_lowercase() as i32 - 97;
        let row = 8 - ((bytes[1] as char) as i32 - 48);
        Vec2D::new(row, col)
    }

    fn format_square(pos: Vec2D) -> String {
        let col = ((pos.y + 97) as u8) as char;
        let row = 8 - pos.x;
        format!("{}{}", col, row)
    }
}