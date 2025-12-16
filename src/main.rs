mod Background;
mod Aox;
mod Pice;
mod TextureMap;
mod Board;
mod Move;
mod PicePosibleMoves;
mod Game;
mod Rendering;
mod PiceSelectMenu;
mod GameOverMenu;
mod Button;

use raylib::prelude::*;
use crate::Aox::{get_board_draw_positions, Vec2D};
use crate::Background::draw_background;

fn main() {
    let width: i32 = 1000;
    let (mut rl, thread) = raylib::init()
        .size(width, 1100)
        .title("Raylib Rust Chess Board")
        .build();

    let mut game = Game::Game::new(&mut rl, &thread);
    let mut game_over_menu = GameOverMenu::GameOverMenu::new();
    let mut should_reset = false;

    while !rl.window_should_close() {
        if should_reset {
            game = Game::Game::new(&mut rl, &thread);
            should_reset = false;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        game.render(&mut d);

        if game.game_over_state != -1 true{
            game_over_menu.render(&mut d, game.game_over_state);

            if game_over_menu.update(&mut d) {
                should_reset = true;
            }
        } else {
            game.update(&mut d);
        }

        d.draw_fps(10, 10);
    }
}