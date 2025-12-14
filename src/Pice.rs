use raylib::prelude::RaylibDrawHandle;
use crate::Aox::{Line2D, Vec2D};
use crate::Rendering::draw_centered_texture;
use crate::TextureMap::TextureMap;

const AnimationSpeed: f32 = 50.0;

#[derive(Clone)]
pub struct Pice {
    pub(crate) pos: Vec2D,
    pub TextureID: i32,
    animation: f32,
    LineBuffer: Line2D,
    pub is_moving: bool,
    pub(crate) side: bool,
    pub is_taken:bool
}

impl Pice {
    pub fn new(pos: Vec2D, texture_id: i32, side: bool) -> Self {
        Self {
            pos,
            TextureID: texture_id,
            animation: 0.0,
            LineBuffer: Line2D::new(Vec2D::new(0, 0), Vec2D::new(0, 0)),
            is_moving: false,
            side,
            is_taken:false
        }
    }

    pub fn render(&self, rl: &mut RaylibDrawHandle, positions: &Vec<Vec<Vec2D>>, t: &TextureMap) {
        let pos = self.compute_position(positions);
        let height = 95;
        let mut opacity =1.0;
        if(self.is_taken){
            opacity=self.animation;
        }
        if self.side {
            draw_centered_texture(rl, &t.white_textures[self.TextureID as usize], pos, height, false,opacity as f64);
        } else {
            draw_centered_texture(rl, &t.black_textures[self.TextureID as usize], pos, height, false,opacity as f64);
        }
    }
    pub fn take(&mut self){
        self.is_taken=true;
        self.animation=1.0;;
    }

    pub fn compute_position(&self, positions: &Vec<Vec<Vec2D>>) -> Vec2D {
        let pos: Vec2D = positions[self.pos.x as usize][self.pos.y as usize];
        if !self.is_moving {
            return pos;
        }
        let line = Line2D::new(
            positions[self.LineBuffer.start.x as usize][self.LineBuffer.start.y as usize],
            positions[self.LineBuffer.end.x as usize][self.LineBuffer.end.y as usize],
        );
        return line.point_at(self.animation);
    }

    pub fn update_animation(&mut self, rl: &mut RaylibDrawHandle) {
        if(self.is_taken){
            self.animation -= AnimationSpeed * rl.get_frame_time();
            if self.animation <= 0.0 {
                self.animation = 0.0;
            }
            return;
        }
        if self.is_moving {
            self.animation += AnimationSpeed * rl.get_frame_time();
        }
        if self.animation >= 1.0 {
            self.animation = 0.0;
            self.is_moving = false;
            self.pos = Vec2D::new(self.LineBuffer.end.x, self.LineBuffer.end.y);
        }
    }

    pub fn update(&mut self, rl: &mut RaylibDrawHandle) {
        self.update_animation(rl);
    }
}