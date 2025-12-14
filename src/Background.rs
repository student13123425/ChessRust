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

pub fn draw_background(d: &mut RaylibDrawHandle, start_x: i32, start_y: i32, board_size: i32){
    draw_chess_board(d, start_x, start_y, board_size);
}