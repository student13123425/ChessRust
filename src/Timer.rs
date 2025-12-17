use raylib::prelude::*;
use crate::Aox::Vec2D;

pub struct Timer {
    time_left: f32,
    initial_time: f32,
    running: bool,
}

impl Timer {
    pub fn new(time_str: &str) -> Self {
        let parts: Vec<&str> = time_str.split(':').collect();
        let minutes: f32 = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let seconds: f32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let total_seconds = minutes * 60.0 + seconds;

        Self {
            time_left: total_seconds,
            initial_time: total_seconds,
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn reset(&mut self) {
        self.time_left = self.initial_time;
        self.running = false;
    }

    pub fn update(&mut self, rl: &RaylibHandle) -> bool {
        if self.running {
            if self.time_left > 0.0 {
                self.time_left -= rl.get_frame_time();
                if self.time_left <= 0.0 {
                    self.time_left = 0.0;
                    self.running = false;
                    return true;
                }
            } else {
                return true;
            }
        }
        false
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, center: Vec2D, font_size: i32) {
        let minutes = (self.time_left / 60.0).floor() as i32;
        let seconds = (self.time_left % 60.0).floor() as i32;
        let time_text = format!("{:02}:{:02}", minutes, seconds);

        let text_width = d.measure_text(&time_text, font_size);
        let padding = 10;
        let rect_width = text_width + padding * 2;
        let rect_height = font_size + padding * 2;
        
        let rect_x = center.x - rect_width / 2;
        let rect_y = center.y - rect_height / 2;

        let rect = Rectangle {
            x: rect_x as f32,
            y: rect_y as f32,
            width: rect_width as f32,
            height: rect_height as f32,
        };

        d.draw_rectangle_rounded(rect, 0.5, 10, Color::WHITE);
        d.draw_text(&time_text, rect_x + padding, rect_y + padding, font_size, Color::BLACK);
    }
}
