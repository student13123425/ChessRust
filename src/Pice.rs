use raylib::prelude::RaylibDrawHandle;
use crate::Aox::{Line2D, Vec2D};
const AnimationSpeed:f32=50.0;
pub struct Pice {
    pos:Vec2D,
    TextureID:i32,
    animation:f32,
    LineBuffer:Line2D,
    is_moving:bool,
    side:bool
}

impl Pice {
    pub fn new(pos: Vec2D, texture_id: i32,side:bool) -> Self {
        Self {
            pos,
            TextureID: texture_id,
            animation: 0.0,
            LineBuffer: Line2D::new(Vec2D::new(0,0),Vec2D::new(0,0)),
            is_moving:false,
            side:side
        }
    }

    pub fn render(&self,rl:&mut RaylibDrawHandle,positions:Vec<Vec<Vec2D>>){
        let pos:Vec2D = positions[self.pos.x as usize][self.pos.y as usize];
    }
    pub fn update(&mut self,rl:&mut RaylibDrawHandle){
        if self.is_moving{
            self.animation+=AnimationSpeed*rl.get_frame_time();
        }
    }
}