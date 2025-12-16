use std::thread::Thread;
use raylib::drawing::RaylibDrawHandle;
use raylib::{RaylibHandle, RaylibThread};
use raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT;
use crate::Aox::{get_click_rect, get_string_repeted_count, Rect2D, Vec2D};
use crate::Background::draw_background;
use crate::Board::Board;
use crate::Move::{render_history, Move};
use crate::Pice::Pice;
use crate::PicePosibleMoves::PosibleMoves;
use crate::PiceSelectMenu::PiceSelectMenu;
use crate::TextureMap::TextureMap;

pub struct Game {
    pub board: Board,
    pub selected_pice:Vec2D,
    pub texture_map: TextureMap,
    pub moves:PosibleMoves,
    pub click_rects:Vec<Vec<Rect2D>>,
    pub side:bool,
    pub hystory:Vec<Move>,
    pub pice_select_menu:PiceSelectMenu,
    pub moveing_pice_buffer:i32,
    pub hystory_of_state:Vec<String>,
    pub game_over_state:i32,
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
            pice_select_menu:PiceSelectMenu::new(),
            moveing_pice_buffer:0,
            hystory_of_state:Vec::new(),
            game_over_state: -1
        }
    }
    pub fn render(&mut self, d: &mut RaylibDrawHandle){
        draw_background(d,0,0,1000);
        self.moves.render(d,&self.board.positions);
        self.board.render(d,&self.texture_map);
        let is_select= self.get_is_to_select_transform();
        if is_select != 0 {
            self.pice_select_menu.render(d,&self.texture_map,is_select==2);
        }
        render_history(d,&self.hystory);
    }
    pub fn get_is_to_select_transform(&self)->i32{
        for p in self.board.WhitePices.iter(){
            if p.pos.x==0 && p.TextureID == 5 {
                return 1
            }
        }
        for p in self.board.BlackPices.iter(){
            if p.pos.x==7 && p.TextureID == 5 {
                return 2
            }
        }
        return 0;
    }
    pub fn process_pawn_select_pice(&mut self,side:bool,pice_id:i32){
        self.board.transform_pawn(side, pice_id);
        if let Some(last_move) = self.hystory.last_mut() {
            last_move.transform_pawn = pice_id;
        }
    }
    pub fn update(&mut self,d: &mut RaylibDrawHandle){
        let is_select= self.get_is_to_select_transform();
        if is_select!=0 {
            let selected=self.pice_select_menu.update(d);
            if selected!=-1 {
                self.process_pawn_select_pice(is_select==2,selected);
            }else{
                return
            }
        }
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
        self.moves.compute_moves(pice,&self.board,true);
    }
    pub fn process_deselect(&mut self){
        self.selected_pice=Vec2D::new(-1,-1);
        self.moves.clear();
    }
    pub fn process_pice_start_move(&mut self, d: &mut RaylibDrawHandle) -> bool {
        let mouse_pos:Vec2D=Vec2D::new(d.get_mouse_x(),d.get_mouse_y());
        for i in 0..self.moves.moves.len(){
            let move_obj = &self.moves.moves[i];
            let target_pos = move_obj.get_end_pos();

            if target_pos.x >= 0 && target_pos.x < 8 && target_pos.y >= 0 && target_pos.y < 8 {
                let rect:&Rect2D=&self.click_rects[target_pos.x as usize][target_pos.y as usize];
                let is_hover=rect.contains(mouse_pos);
                let is_cliked=d.is_mouse_button_released(MOUSE_BUTTON_LEFT);
                if is_hover && is_cliked {
                    self.process_move(move_obj.clone());
                    self.process_deselect();
                    return true;
                }
            }
        }
        return false;
    }
    pub fn get_if_draw(&mut self)->bool{
        if self.moveing_pice_buffer > 100 || self.board.get_if_low_material() || get_string_repeted_count(&mut self.hystory_of_state) >= 3 {
            return true
        }
        return false
    }
    pub fn process_move(&mut self, move_obj: Move){
        let value_at_point=self.board.get_value_at_point(move_obj.get_start_pos());
        let is_pawn=value_at_point==5||value_at_point==13;
        self.hystory_of_state.push(self.board.to_string(self.side));
        let is_take=self.board.execute_move(&move_obj);
        self.moveing_pice_buffer+=1;
        if is_take || is_pawn {
            self.moveing_pice_buffer=0;
        }
        self.side = !self.side;
        self.hystory.push(move_obj);

        if self.get_if_draw() {
            self.game_over_state = 3;
        } else {
             let state = self.board.is_checkmate_or_stale_mate(self.side);
             if state != -1 {
                 self.game_over_state = state;
             }
        }
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