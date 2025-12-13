use std::thread::Thread;
use raylib::drawing::RaylibDrawHandle;
use raylib::{RaylibHandle, RaylibThread};
use crate::Aox::Vec2D;
use crate::Background::draw_background;
use crate::Board::Board;
use crate::TextureMap::TextureMap;

pub struct Game {
    pub board: Board,
    pub selected_pice:Vec2D,
    pub texture_map: TextureMap
}

impl Game {
    pub fn new(rl:&mut RaylibHandle,raylib_thread: &RaylibThread) -> Self {
        Self {
            board: Board::new(),
            selected_pice:Vec2D::new(-1,-1),
            texture_map:TextureMap::new(rl,raylib_thread)
        }
    }
    pub fn render(&mut self, d: &mut RaylibDrawHandle){
        draw_background(d,0,0,1000);
    }
    pub fn update(&mut self){
        
    }
}