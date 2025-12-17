use raylib::prelude::*;
use crate::Button::Button;

pub struct GameOverMenu {
    pub reset_button: Button,
    pub exit_button: Button,
}

impl GameOverMenu {
    pub fn new() -> Self {
        let play_green = Color::new(129, 182, 76, 255);
        let play_green_hover = Color::new(169, 212, 116, 255);
        
        let exit_gray = Color::new(50, 50, 50, 255);
        let exit_gray_hover = Color::new(70, 70, 70, 255);
        
        Self {
            reset_button: Button::new(
                350.0, 500.0, 300.0, 70.0, 
                "Play Again", 
                play_green, 
                play_green_hover, 
                Color::WHITE
            ),
            exit_button: Button::new(
                350.0, 600.0, 300.0, 70.0, 
                "Exit", 
                exit_gray, 
                exit_gray_hover, 
                Color::WHITE
            ),
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, game_over_state: i32) {
        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();

        d.draw_rectangle(0, 0, screen_width, screen_height, Color::new(0, 0, 0, 150));

        let modal_width = 500;
        let modal_height = 500;
        let modal_x = (screen_width - modal_width) / 2;
        let modal_y = (screen_height - modal_height) / 2;

        let modal_rect = Rectangle::new(
            modal_x as f32, 
            modal_y as f32, 
            modal_width as f32, 
            modal_height as f32
        );

        d.draw_rectangle_rounded(modal_rect, 0.1, 10, Color::new(38, 37, 34, 255)); 

        let text = match game_over_state {
            0 => "Stalemate",
            1 => "White Wins",
            2 => "Black Wins",
            3 => "Draw",
            _ => "Game Over",
        };

        let title_font_size = 50;
        let text_width = d.measure_text(text, title_font_size);
        let text_x = modal_x + (modal_width - text_width) / 2;
        let text_y = modal_y + 80;

        d.draw_text(text, text_x, text_y, title_font_size, Color::WHITE);

        self.reset_button.render(d);
        self.exit_button.render(d);
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) -> bool {
        if self.reset_button.update(d) {
            return true;
        }

        if self.exit_button.update(d) {
            std::process::exit(0);
        }
        
        return false;
    }
}