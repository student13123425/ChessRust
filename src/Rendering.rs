use raylib::drawing::RaylibDraw;
use raylib::prelude::{Color, RaylibDrawHandle, Rectangle, Texture2D, Vector2};
use crate::Aox::Vec2D;

pub fn draw_centered_texture(
    d: &mut RaylibDrawHandle,
    texture: &Texture2D,
    center: Vec2D,
    s: i32,
    is_width: bool,
) {
    if center.is_null() {
        return;
    }

    let tex_w = texture.width as f32;
    let tex_h = texture.height as f32;
    let target_s = s as f32;

    let (final_w, final_h) = if is_width {
        let scale = target_s / tex_w;
        (target_s, tex_h * scale)
    } else {
        let scale = target_s / tex_h;
        (tex_w * scale, target_s)
    };

    let source_rec = Rectangle::new(0.0, 0.0, tex_w, tex_h);

    let dest_rec = Rectangle::new(
        center.x as f32,
        center.y as f32,
        final_w,
        final_h
    );

    let origin = Vector2::new(final_w / 2.0, final_h / 2.0);

    d.draw_texture_pro(
        texture,
        source_rec,
        dest_rec,
        origin,
        0.0,
        Color::WHITE,
    );
}