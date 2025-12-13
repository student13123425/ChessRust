use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use crate::Aox::{Rect2D, Vec2D};

pub fn draw_chess_board(d: &mut RaylibDrawHandle, start_x: i32, start_y: i32, board_size: i32) {
    let tile_size = board_size / 8;
    let light_color = Color::new(238, 238, 210, 255);
    let dark_color = Color::new(118, 150, 86, 255);
    for row in 0..8 {
        for col in 0..8 {
            let color = if (row + col) % 2 == 0 {
                light_color
            } else {
                dark_color
            };
            let x = start_x + (col * tile_size);
            let y = start_y + (row * tile_size);
            d.draw_rectangle(x, y, tile_size, tile_size, color);
        }
    }
}



fn draw_chess_notation(d: &mut RaylibDrawHandle, start_x: i32, start_y: i32, board_size: i32) {
    let tile_size = board_size / 8;
    let font_size = 20;
    let text_color = Color::BLACK;

    for i in 0..8 {
        let rank_num = 8 - i;
        let rank_str = format!("{}", rank_num);
        let rank_x = start_x - 25;
        let rank_y = start_y + (i * tile_size) + (tile_size / 2) - 10;

        d.draw_text(&rank_str, rank_x, rank_y, font_size, text_color);

        let file_char = (b'A' + i as u8) as char;
        let file_str = format!("{}", file_char);
        let file_x = start_x + (i * tile_size) + (tile_size / 2) - 5;
        let file_y = start_y + board_size + 10;

        d.draw_text(&file_str, file_x, file_y, font_size, text_color);
    }
}

pub fn draw_background(d: &mut RaylibDrawHandle, start_x: i32, start_y: i32, board_size: i32){
    draw_chess_notation(d, start_x, start_y, board_size);
    draw_chess_board(d, start_x, start_y, board_size);
}