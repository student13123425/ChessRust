mod Background;
mod Aox;

use raylib::prelude::*;
use crate::Background::draw_background;

fn main() {
    let width:i32=1000;
    let (mut rl, thread) = raylib::init()
        .size(width, 1100)
        .title("Raylib Rust Chess Board")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        draw_background(&mut d,0,0,width);
    }
}