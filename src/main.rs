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
mod AudioPlayer;
mod MainMenu;
mod Timer;
mod TimeSelectMenu;

use raylib::prelude::*;
use crate::Aox::{get_board_draw_positions, Vec2D};
use crate::Background::draw_background;

fn main() {
    let width: i32 = 1000;
    let (mut rl, thread) = raylib::init()
        .size(width, 1100)
        .title("Raylib Rust Chess Board")
        .build();

    let mut current_time_selection = 4;
    let mut game = Game::Game::new(&mut rl, &thread, current_time_selection);
    let mut game_over_menu = GameOverMenu::GameOverMenu::new();
    let mut main_menu = MainMenu::MainMenu::new();
    let mut time_select_menu = TimeSelectMenu::TimeSelectMenu::new();
    
    let mut should_reset = false;
    let mut in_menu = true;
    let mut in_time_select = false;

    while !rl.window_should_close() {
        if should_reset {
            game = Game::Game::new(&mut rl, &thread, current_time_selection);
            should_reset = false;
        }

        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        if in_menu {
            main_menu.render(&mut d);
            let menu_action = main_menu.update(&mut d);
            
            if menu_action == 0 {
                in_menu = false;
                in_time_select = true;
            } else if menu_action == 1 {
            }
        } else if in_time_select {
            time_select_menu.render(&mut d);
            let selection = time_select_menu.update(&mut d);
            if selection != -1 {
                current_time_selection = selection;
                in_time_select = false;
                should_reset = true;
            }
        } else {
            game.render(&mut d);

            if game.game_over_state != -1 {
                game_over_menu.render(&mut d, game.game_over_state);

                if game_over_menu.update(&mut d) {
                    should_reset = true;
                }
            } else {
                game.update(&mut d, dt);
            }
        }

        d.draw_fps(10, 10);
    }
}