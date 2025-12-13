mod Background;
mod Aox;
mod Pice;
mod TextureMap;
mod Board;
mod Move;
mod PicePosibleMoves;
mod Game;
mod Rendering;

use raylib::prelude::*;
use crate::Aox::{get_board_draw_positions, Vec2D};
use crate::Background::draw_background;

fn main() {
    let width:i32=1000;
    let (mut rl, thread) = raylib::init()
        .size(width, 1100)
        .title("Raylib Rust Chess Board")
        .build();
    let mut game:Game::Game=Game::Game::new(&mut rl,&thread);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        game.render(&mut d);
        game.update(&mut d);
    }
}