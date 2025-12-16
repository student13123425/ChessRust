use raylib::prelude::*;
use crate::Aox::{Rect2D, Vec2D};

pub struct GameOverMenu {
    pub reset_button: Rect2D,
    pub exit_button: Rect2D,
}

impl GameOverMenu {
    pub fn new() -> Self {
        Self {
            // Rect2D::new(x, y, width, height)
            // Creating buttons 200px wide and 100px tall centered horizontally
            reset_button: Rect2D::new(400, 500, 200, 100),
            exit_button: Rect2D::new(400, 700, 200, 100),
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, game_over_state: i32) {
        let text = match game_over_state {
            0 => "Stale Mate",
            1 => "White Wins",
            2 => "Black Wins",
            3 => "Draw",
            _ => "Game Over",
        };

        // Fix 1: Use 'd.measure_text' instead of global 'measure_text'
        let text_width = d.measure_text(text, 50);
        d.draw_text(text, 500 - (text_width / 2), 200, 50, Color::BLACK);

        // Fix 2: Use .x, .y, .width, .height from Aox.rs instead of .min/.max

        // Draw Reset Button
        d.draw_rectangle(
            self.reset_button.x,
            self.reset_button.y,
            self.reset_button.width,
            self.reset_button.height,
            Color::BLUE
        );
        d.draw_text("Play Again", self.reset_button.x + 20, self.reset_button.y + 35, 30, Color::WHITE);

        // Draw Exit Button
        d.draw_rectangle(
            self.exit_button.x,
            self.exit_button.y,
            self.exit_button.width,
            self.exit_button.height,
            Color::RED
        );
        d.draw_text("Exit", self.exit_button.x + 70, self.exit_button.y + 35, 30, Color::WHITE);
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) -> bool {
        let mouse_pos = Vec2D::new(d.get_mouse_x(), d.get_mouse_y());

        if d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.reset_button.contains(mouse_pos) {
                return true;
            }
            if self.exit_button.contains(mouse_pos) {
                std::process::exit(0);
            }
        }
        return false;
    }
}