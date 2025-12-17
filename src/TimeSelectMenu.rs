use raylib::prelude::*;
use crate::Button::Button;

pub struct TimeSelectMenu {
    pub min_1_button: Button,
    pub min_3_button: Button,
    pub min_5_button: Button,
    pub min_10_button: Button,
    pub min_30_button: Button,
}

impl TimeSelectMenu {
    pub fn new() -> Self {
        let button_width = 300.0;
        let button_height = 70.0;
        let screen_width = 1000.0;
        let screen_height = 1100.0; // Assuming same as MainMenu context or derived
        let center_x = (screen_width - button_width) / 2.0;

        let gray = Color::new(50, 50, 50, 255);
        let gray_hover = Color::new(70, 70, 70, 255);
        let text_color = Color::WHITE;
        
        let mut start_y = 250.0; // Started a bit higher than MainMenu to fit 5 buttons
        let spacing = 90.0; // Slightly tighter spacing

        Self {
            min_1_button: Button::new(
                center_x, start_y, button_width, button_height,
                "1 Minute", gray, gray_hover, text_color,
            ),
            min_3_button: Button::new(
                center_x, start_y + spacing, button_width, button_height,
                "3 Minutes", gray, gray_hover, text_color,
            ),
            min_5_button: Button::new(
                center_x, start_y + spacing * 2.0, button_width, button_height,
                "5 Minutes", gray, gray_hover, text_color,
            ),
            min_10_button: Button::new(
                center_x, start_y + spacing * 3.0, button_width, button_height,
                "10 Minutes", gray, gray_hover, text_color,
            ),
            min_30_button: Button::new(
                center_x, start_y + spacing * 4.0, button_width, button_height,
                "30 Minutes", gray, gray_hover, text_color,
            ),
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::new(48, 46, 43, 255)); 

        let title_text = "Select Time";
        let title_font_size = 60;
        let title_width = d.measure_text(title_text, title_font_size);
        let title_x = (d.get_screen_width() - title_width) / 2;
        let title_y = 100;

        d.draw_text(title_text, title_x, title_y, title_font_size, Color::WHITE);

        self.min_1_button.render(d);
        self.min_3_button.render(d);
        self.min_5_button.render(d);
        self.min_10_button.render(d);
        self.min_30_button.render(d);
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) -> i32 {
        if self.min_1_button.update(d) {
            return 0; // 1 min
        }
        if self.min_3_button.update(d) {
            return 2; // 3 min
        }
        if self.min_5_button.update(d) {
            return 3; // 5 min
        }
        if self.min_10_button.update(d) {
            return 4; // 10 min
        }
        if self.min_30_button.update(d) {
            return 5; // 30 min
        }
        return -1;
    }
}
