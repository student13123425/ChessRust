use std::thread::Thread;
use raylib::drawing::RaylibDrawHandle;
use raylib::{RaylibHandle, RaylibThread};
use raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT;
use crate::Aox::{get_click_rect, Rect2D, Vec2D};
use crate::Background::draw_background;
use crate::Board::Board;
use crate::Pice::Pice;
use crate::PicePosibleMoves::PosibleMoves;
use crate::TextureMap::TextureMap;

pub struct Game {
    pub board: Board,
    pub selected_pice:Vec2D,
    pub texture_map: TextureMap,
    pub moves:PosibleMoves,
    pub click_rects:Vec<Vec<Rect2D>>,
    pub side:bool
}

impl Game {
    pub fn new(rl:&mut RaylibHandle,raylib_thread: &RaylibThread) -> Self {
        Self {
            board: Board::new(),
            selected_pice:Vec2D::new(-1,-1),
            texture_map:TextureMap::new(rl,raylib_thread),
            moves:PosibleMoves::new(),
            click_rects:get_click_rect(0,0,1000),
            side:false
        }
    }
    pub fn render(&mut self, d: &mut RaylibDrawHandle){
        draw_background(d,0,0,1000);
        self.board.render(d,&self.texture_map);
        self.moves.render(d,&self.board.positions);
    }
    pub fn update(&mut self,d: &mut RaylibDrawHandle){
        self.moves.update(d);
        self.process_pice_select(d);
    }
    pub fn process_click(&mut self,point:Vec2D,pice: &Pice){
        self.selected_pice=Vec2D::new(point.x,point.y);
        self.moves.compute_moves(pice,&self.board);
    }
    pub fn process_deselect(&mut self){
        self.selected_pice=Vec2D::new(-1,-1);
        self.moves.clear();
    }
    pub fn process_pice_select(&mut self, d: &mut RaylibDrawHandle){
        let mut values=self.board.get_pice_side(self.side);
        let mut data:Vec<Vec2D>=Vec::new();
        for v in 0..(values).len(){
            data.push(Vec2D::new(values[v].pos.x,values[v].pos.y))
        }
        let mouse_pos:Vec2D=Vec2D::new(d.get_mouse_x(),d.get_mouse_y());
        for i in 0..8{
            for j in 0..8{
                let point=Vec2D::new(i,j);
                let is_hover=self.click_rects[i as usize][j as usize].contains(mouse_pos);
                let is_cliked=d.is_mouse_button_released(MOUSE_BUTTON_LEFT);
                if(is_cliked&&is_hover){
                    for i in 0..data.len(){
                        if(data[i].compair(&point)) {
                            self.process_click(point,&values[i]);
                            return
                        }
                        self.process_deselect()
                    }

                }
            }
        }
    }
}