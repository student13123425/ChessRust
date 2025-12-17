use raylib::prelude::*;
use crate::Button::Button;

pub struct MainMenu {
    pub pvp_button: Button,
    pub pvai_button: Button,
    pub quit_button: Button,
}

impl MainMenu {
    pub fn new() -> Self {
        let button_width = 300.0;
        let button_height = 70.0;
        let screen_width = 1000.0;
        let screen_height = 1100.0;
        let center_x = (screen_width - button_width) / 2.0;
        let play_green = Color::new(129, 182, 76, 255);
        let play_green_hover = Color::new(169, 212, 116, 255);
        
        let gray = Color::new(50, 50, 50, 255);
        let gray_hover = Color::new(70, 70, 70, 255);

        let start_y = 400.0;
        let spacing = 100.0;

        Self {
            pvp_button: Button::new(
                center_x, start_y, button_width, button_height,
                "Player vs Player",
                play_green,
                play_green_hover,
                Color::WHITE,
            ),
            pvai_button: Button::new(
                center_x, start_y + spacing, button_width, button_height,
                "Player vs AI",
                gray,
                gray_hover,
                Color::WHITE,
            ),
            quit_button: Button::new(
                center_x, start_y + spacing * 2.0, button_width, button_height,
                "Quit",
                gray,
                gray_hover,
                Color::WHITE,
            ),
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::new(48, 46, 43, 255)); 

        let title_text = "Chess Rust";
        let title_font_size = 80;
        let title_width = d.measure_text(title_text, title_font_size);
        let title_x = (d.get_screen_width() - title_width) / 2;
        let title_y = 200;

        d.draw_text(title_text, title_x, title_y, title_font_size, Color::WHITE);

        self.pvp_button.render(d);
        self.pvai_button.render(d);
        self.quit_button.render(d);
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) -> i32 {
        if self.pvp_button.update(d) {
            return 0;
        }

        if self.pvai_button.update(d) {
            return 1;
        }

        if self.quit_button.update(d) {
            std::process::exit(0);
        }

        return -1;
    }
}
