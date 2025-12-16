use raylib::prelude::*;
use crate::Aox::Vec2D;

pub struct Button {
    pub rect: Rectangle,
    pub text: String,
    pub color: Color,
    pub hover_color: Color,
    pub text_color: Color,
    pub is_hovered: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: &str, color: Color, hover_color: Color, text_color: Color) -> Self {
        Self {
            rect: Rectangle::new(x, y, width, height),
            text: text.to_string(),
            color,
            hover_color,
            text_color,
            is_hovered: false,
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        let draw_color = if self.is_hovered { self.hover_color } else { self.color };
        
        d.draw_rectangle_rounded(
            self.rect, 
            0.3, 
            10,
            draw_color
        );
        
        
        let font_size = 30;
        let text_width = d.measure_text(&self.text, font_size);
        let text_x = self.rect.x as i32 + (self.rect.width as i32 - text_width) / 2;
        let text_y = self.rect.y as i32 + (self.rect.height as i32 - font_size) / 2;
        
        d.draw_text(&self.text, text_x, text_y, font_size, self.text_color);
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) -> bool {
        let mouse_pos = d.get_mouse_position();
        
        self.is_hovered = self.rect.check_collision_point_rec(mouse_pos);

        if self.is_hovered && d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            return true;
        }
        
        return false;
    }
}
