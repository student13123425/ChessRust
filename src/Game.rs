use std::thread::Thread;
use raylib::drawing::RaylibDrawHandle;
use raylib::{RaylibHandle, RaylibThread};
use raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT;
use crate::Aox::{get_click_rect, Rect2D, Vec2D};
use crate::Background::draw_background;
use crate::Board::Board;
use crate::Move::Move;
use crate::Pice::Pice;
use crate::PicePosibleMoves::PosibleMoves;
use crate::TextureMap::TextureMap;

pub struct Game {
    pub board: Board,
    pub selected_pice:Vec2D,
    pub texture_map: TextureMap,
    pub moves:PosibleMoves,
    pub click_rects:Vec<Vec<Rect2D>>,
    pub side:bool,
    pub hystory:Vec<Move>,
}

impl Game {
    pub fn new(rl:&mut RaylibHandle,raylib_thread: &RaylibThread) -> Self {
        Self {
            board: Board::new(),
            selected_pice:Vec2D::new(-1,-1),
            texture_map:TextureMap::new(rl,raylib_thread),
            moves:PosibleMoves::new(),
            click_rects:get_click_rect(0,0,1000),
            side:true,
            hystory:Vec::new(),
        }
    }
    pub fn render(&mut self, d: &mut RaylibDrawHandle){
        draw_background(d,0,0,1000);
        self.moves.render(d,&self.board.positions);
        self.board.render(d,&self.texture_map);

    }
    pub fn update(&mut self,d: &mut RaylibDrawHandle){
        self.board.update(d);
        self.moves.update(d);
        if self.board.get_is_pice_moving() {
            return;
        }
        let moved = self.process_pice_start_move(d);
        if !moved {
            self.process_pice_select(d);
        }
    }
    pub fn process_click(&mut self,point:Vec2D,pice: &Pice){
        self.selected_pice=Vec2D::new(point.x,point.y);
        self.moves.compute_moves(pice,&self.board);
    }
    pub fn process_deselect(&mut self){
        self.selected_pice=Vec2D::new(-1,-1);
        self.moves.clear();
    }
    pub fn process_pice_start_move(&mut self, d: &mut RaylibDrawHandle) -> bool {
        let mouse_pos:Vec2D=Vec2D::new(d.get_mouse_x(),d.get_mouse_y());
        for i in 0..self.moves.moves.len(){
            let pos=&self.moves.moves[i];
            let rect:&Rect2D=&self.click_rects[pos.x as usize][pos.y as usize];
            let is_hover=rect.contains(mouse_pos);
            let is_cliked=d.is_mouse_button_released(MOUSE_BUTTON_LEFT);
            if is_hover && is_cliked {
                self.process_move(pos.clone());
                self.process_deselect();
                return true;
            }
        }
        return false;
    }
    pub fn process_move(&mut self,end:Vec2D){
        let start=self.selected_pice;
        let mut pices=&mut self.board.BlackPices;
        if self.side {
            pices=&mut self.board.WhitePices;
        }
        let mut pice_id:i32=0;
        for p in pices{
            if !p.is_taken {
                let is_selected_pice = p.pos.compair(&start);
                if is_selected_pice {
                    p.move_pice(&end);
                    pice_id = p.TextureID;
                    break;
                }
            }
        }
        self.side=!self.side;
        pices=&mut self.board.BlackPices;
        if self.side {
            pices=&mut self.board.WhitePices;
        }
        for p in pices{
            if !p.is_taken && p.pos.compair(&end) {
                p.take();
                break;
            }
        }
        self.hystory.push(Move::from_pos(Vec2D::new(start.x,start.y),Vec2D::new(end.x,end.y),pice_id,self.side))
    }
    pub fn process_pice_select(&mut self, d: &mut RaylibDrawHandle){
        let values = self.board.get_pice_side(self.side);

        let mouse_pos:Vec2D=Vec2D::new(d.get_mouse_x(),d.get_mouse_y());
        for i in 0..8{
            for j in 0..8{
                let point=Vec2D::new(i,j);
                let is_hover=self.click_rects[i as usize][j as usize].contains(mouse_pos);
                let is_cliked=d.is_mouse_button_released(MOUSE_BUTTON_LEFT);

                if is_cliked && is_hover {
                    let mut clicked_friendly = false;
                    for k in 0..values.len() {
                        if !values[k].is_taken && values[k].pos.compair(&point) {
                            self.process_click(point, &values[k]);
                            clicked_friendly = true;
                            break;
                        }
                    }

                    if !clicked_friendly {
                        self.process_deselect();
                    }
                    return;
                }
            }
        }
    }
}