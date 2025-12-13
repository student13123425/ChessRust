use std::thread::Thread;
use raylib::drawing::RaylibDrawHandle;
use raylib::{RaylibHandle, RaylibThread};
use crate::Aox::{get_click_rect, Rect2D, Vec2D};
use crate::Background::draw_background;
use crate::Board::Board;
use crate::PicePosibleMoves::PosibleMoves;
use crate::TextureMap::TextureMap;

pub struct Game {
    pub board: Board,
    pub selected_pice:Vec2D,
    pub texture_map: TextureMap,
    pub moves:PosibleMoves,
    pub click_rects:Vec<Vec<Rect2D>>
}

impl Game {
    pub fn new(rl:&mut RaylibHandle,raylib_thread: &RaylibThread) -> Self {
        Self {
            board: Board::new(),
            selected_pice:Vec2D::new(-1,-1),
            texture_map:TextureMap::new(rl,raylib_thread),
            moves:PosibleMoves::new(),
            click_rects:get_click_rect(0,0,1000);
        }
    }
    pub fn render(&mut self, d: &mut RaylibDrawHandle){
        draw_background(d,0,0,1000);
        self.board.render(d,&self.texture_map);
    }
    pub fn update(&mut self,d: &mut RaylibDrawHandle){
        self.moves.update(d);
    }
}