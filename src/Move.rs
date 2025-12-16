use raylib::prelude::*;
use crate::Aox::Vec2D;

#[derive(Debug, Clone)]
pub struct Move {
    pub start_sq: String,
    pub end_sq: String,
    pub piece_id: i32,
    pub side: bool,
    pub is_castling: bool,
    pub transform_pawn:i32,
    pub is_en_passant: bool
}

impl Move {
    pub fn new(start: String, end: String, piece_id: i32, side: bool, is_castling: bool) -> Self {
        Self {
            start_sq: start,
            end_sq: end,
            piece_id,
            side,
            is_castling,
            transform_pawn:-1,
            is_en_passant: false
        }
    }

    pub fn from_pos(start: Vec2D, end: Vec2D, piece_id: i32, side: bool, is_castling: bool) -> Self {
        let start_sq = Self::format_square(start);
        let end_sq = Self::format_square(end);
        Self {
            start_sq,
            end_sq,
            piece_id,
            side,
            is_castling,
            transform_pawn:-1,
            is_en_passant: false
        }
    }
    pub fn transform_pawn_move(pos:Vec2D,new_value:i32,side:bool)->Self{
        let start_sq=Self::format_square(pos);
        let end_sq=Self::format_square(pos);
        return Self{
            transform_pawn:new_value,
            start_sq,
            end_sq,
            piece_id:5,
            side:side,
            is_castling:false,
            is_en_passant: false
        }
    }
    pub fn to_string(&self) -> String {
        if self.is_castling {
            if self.end_sq.starts_with('g') {
                return "O-O".to_string();
            } else if self.end_sq.starts_with('c') {
                return "O-O-O".to_string();
            }
        }

        let mut move_str = format!("{}{}", self.start_sq, self.end_sq);

        if self.transform_pawn != -1 {
            let promo_char = match self.transform_pawn {
                1 => "q",
                2 => "r",
                3 => "b",
                4 => "n",
                _ => "q",
            };
            move_str.push_str(promo_char);
        }

        move_str
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

pub fn render_history(d: &mut RaylibDrawHandle, history: &Vec<Move>) {
    let font_size = 20;
    let padding = 10;
    let area_start_y = 1000;
    let area_height = 100;
    let text_y = area_start_y + (area_height - font_size) / 2;
    let spacing = 15; // Space between move groups
    let mut current_x = 1000 - padding;
    let mut i = (history.len() as i32) - 1;

    while i >= 0 {
        let text_to_draw;
        let step;

        if i % 2 != 0 {
            let white_move = &history[(i - 1) as usize];
            let black_move = &history[i as usize];
            let move_num = ((i - 1) / 2) + 1;

            text_to_draw = format!("{}. {}{} {}{}",
                                   move_num,
                                   white_move.start_sq, white_move.end_sq,
                                   black_move.start_sq, black_move.end_sq
            );
            step = 2;
        } else {
            let white_move = &history[i as usize];
            let move_num = (i / 2) + 1;

            text_to_draw = format!("{}. {}{}",
                                   move_num,
                                   white_move.start_sq, white_move.end_sq
            );
            step = 1;
        }

        let text_width = d.measure_text(&text_to_draw, font_size);

        if current_x - text_width < padding {
            break; 
        }

        current_x -= text_width;

        d.draw_text(&text_to_draw, current_x, text_y, font_size, Color::BLACK);
        current_x -= spacing;
        i -= step;
    }
}