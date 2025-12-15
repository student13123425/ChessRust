use raylib::drawing::RaylibDrawHandle;
use crate::Aox::{get_board_draw_positions, Vec2D};
use crate::Pice::Pice;
use crate::PicePosibleMoves::PosibleMoves;
use crate::TextureMap::TextureMap;
use crate::Move::Move;

#[derive(Clone)]
pub struct Board {
    pub BlackPices: Vec<Pice>,
    pub WhitePices: Vec<Pice>,
    pub positions: Vec<Vec<Vec2D>>
}

impl Board {
    pub fn new() -> Board {
        let mut black_pices = Vec::new();
        let mut white_pices = Vec::new();
        let pos = get_board_draw_positions(0, 0, 1000);
        let back_rank_ids = [2, 4, 3, 1, 0, 3, 4, 2];
        for (col, &id) in back_rank_ids.iter().enumerate() {
            black_pices.push(Pice::new(Vec2D::new(0, col as i32), id, false));
        }
        for col in 0..8 {
            black_pices.push(Pice::new(Vec2D::new(1, col as i32), 5, false));
        }
        for (col, &id) in back_rank_ids.iter().enumerate() {
            white_pices.push(Pice::new(Vec2D::new(7, col as i32), id, true));
        }
        for col in 0..8 {
            white_pices.push(Pice::new(Vec2D::new(6, col as i32), 5, true));
        }
        Self {
            BlackPices: black_pices,
            WhitePices: white_pices,
            positions: pos,
        }
    }

    pub fn from_state(state: Vec<Vec<i32>>) -> Board {
        let mut black_pices = Vec::new();
        let mut white_pices = Vec::new();
        let pos = get_board_draw_positions(0, 0, 1000);

        for (r, row) in state.iter().enumerate() {
            for (c, &id) in row.iter().enumerate() {
                if id == -1 {
                    continue;
                }

                let position = Vec2D::new(r as i32, c as i32);

                if id < 8 {
                    white_pices.push(Pice::new(position, id, true));
                } else {
                    black_pices.push(Pice::new(position, id - 8, false));
                }
            }
        }

        Self {
            BlackPices: black_pices,
            WhitePices: white_pices,
            positions: pos,
        }
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        for pice in &mut self.WhitePices {
            pice.update(d);
        }
        for pice in &mut self.BlackPices {
            pice.update(d);
        }
    }
    pub fn get_pice_side(&self,side:bool)->Vec<Pice>{
        let mut out=Vec::new();
        let mut values=&self.BlackPices;
        if side {
            values = &self.WhitePices;
        }
        for pice in values{
            out.push(pice.clone());
        }
        return out;
    }
    pub fn get_board_state(&self) -> Vec<Vec<i32>> {
        let mut board = vec![vec![-1; 8]; 8];
        for pice in &self.WhitePices {
            if pice.is_taken {
                continue;
            }
            let r = pice.pos.x as usize;
            let c = pice.pos.y as usize;
            if r < 8 && c < 8 {
                board[r][c] = pice.TextureID;
            }
        }
        for pice in &self.BlackPices {
            if pice.is_taken {
                continue;
            }
            let r = pice.pos.x as usize;
            let c = pice.pos.y as usize;
            if r < 8 && c < 8 {
                board[r][c] = pice.TextureID + 8;
            }
        }
        return board
    }
    pub fn is_checkmate_or_stale_mate(&self, side: bool) -> i32 {
        let is_check = self.is_check();
        let mut pices = &self.WhitePices;
        if !side {
            pices = &self.BlackPices;
        }

        let mut moves = PosibleMoves::new();
        for pice in pices {
            moves.compute_moves(pice, &self, true);
            for m in &moves.moves {
                let side = m.side;
                let is_check_local = self.is_move_resoult_in_check(m, side);
                if !is_check_local {
                    return -1;
                }
            }
        }
        return is_check;
    }
    pub fn is_move_resoult_in_check(&self,m:&Move,side:bool)->bool{
        let mut b=self.clone();
        b.execute_move(m);
        let v=b.is_check();
        if(side==true&&v==1){
            return true;
        }
        if(side==false&&v==2){
            return true;
        }
        return false
    }
    pub fn is_check(&self,)->i32{
        let mut kings:Vec<Vec2D>=vec![];
        for pice in &self.WhitePices{
            if pice.TextureID==0 {
                kings.push(Vec2D::new(pice.pos.x,pice.pos.y));
            }
        }
        for pice in &self.BlackPices{
            if pice.TextureID==0 {
                kings.push(Vec2D::new(pice.pos.x,pice.pos.y));
            }
        }
        let mut posbile_moves=PosibleMoves::new();
        for pice in &self.WhitePices{
            posbile_moves.compute_moves(pice,self,true);
            for pos in &posbile_moves.moves{
                if kings[1].compair(&pos.get_end_pos()) {
                    return 2;
                }
            }
        }
        for pice in &self.BlackPices{
            posbile_moves.compute_moves(pice,self,true);
            for pos in &posbile_moves.moves{
                if kings[0].compair(&pos.get_end_pos()) {
                    return 1;
                }
            }
        }
        return 0;
    }
    pub fn render(&self,d:&mut RaylibDrawHandle,texture_map:&TextureMap){
        for pice in &self.WhitePices{
            pice.render(d,&self.positions,texture_map)
        }
        for pice in &self.BlackPices{
            pice.render(d,&self.positions,texture_map)
        }
    }
    pub fn get_is_pice_moving(&self)->bool{
        for pice in &self.WhitePices{
            if pice.is_moving {
                return true;
            }
        }
        for pice in &self.BlackPices{
            if pice.is_moving {
                return true;
            }
        }
        return false;
    }
    pub fn get_value_at_point(&self, point: Vec2D) -> i32 {
        let board_state = self.get_board_state();
        let rows = board_state.len();
        if rows == 0 {
            return -1;
        }
        let cols = board_state[0].len();
        let x = point.x;
        let y = point.y;
        if x < 0 || y < 0 || (y as usize) >= rows || (x as usize) >= cols {
            return -1;
        }
        return board_state[y as usize][x as usize];
    }
    pub fn get_pice_counts(&self, side: bool) -> Vec<i32> {
        let mut out = vec![0, 0, 0, 0, 0, 0];
        let state = self.get_board_state();
        for i in 0..8 {
            for j in 0..8 {
                let mut v = state[i][j];
                if v == -1 {
                    continue;
                }
                if side && v >= 8 {
                    continue;
                }
                if !side && v < 8 {
                    continue;
                }
                if v >= 8 {
                    v -= 8;
                }
                out[v as usize] += 1;
            }
        }
        return out;
    }pub fn get_if_low_material(&self) -> bool {
        let a1 = self.get_pice_counts(false);
        let a2 = self.get_pice_counts(true);
        let mut empty_pice_type = vec![0, 0];
        for i in 0..6 {
            if a1[i] == 0 {
                empty_pice_type[0] += 1;
            }
            if a2[i] == 0 {
                empty_pice_type[1] += 1;
            }
        }
        let is_have_materieal_1 = a1[1] > 0 || a1[2] > 0 || a1[5] > 0;
        let is_have_materieal_2 = a2[1] > 0 || a2[2] > 0 || a2[5] > 0;
        if is_have_materieal_1 || is_have_materieal_2 {
            return false;
        }
        if empty_pice_type[0] == 4 || empty_pice_type[1] == 4 {
            let mut buffer = vec![false, false];
            for i in 0..6 {
                if a1[i] > 1 {
                    buffer[0] = true;
                }
                if a2[i] > 1 {
                    buffer[1] = true;
                }
            }
            if buffer[0] || buffer[1] {
                return false;
            }

        }
        if empty_pice_type[0] == 5 || empty_pice_type[1] == 5 {
            return true;
        }
        return false;
    }
    pub fn execute_move(&mut self, move_obj: &Move) ->bool{
        if move_obj.is_castling {
            self.execute_castle_move(move_obj);
            return false;
        }

        let start = move_obj.get_start_pos();
        let end = move_obj.get_end_pos();

        {
            let pices = if move_obj.side { &mut self.WhitePices } else { &mut self.BlackPices };
            for p in pices.iter_mut() {
                if !p.is_taken {
                    let is_selected_pice = p.pos.compair(&start);
                    if is_selected_pice {
                        p.move_pice(&end);
                        break;
                    }
                }
            }
        }
        {
            let opponent_pices = if !move_obj.side { &mut self.WhitePices } else { &mut self.BlackPices };
            for p in opponent_pices {
                if !p.is_taken && p.pos.compair(&end) {
                    p.take();
                    return true;
                }
            }
        }
        return false;
    }

    fn execute_castle_move(&mut self, move_obj: &Move) {
        let mut side_y = 7;
        let mut pices = &mut self.WhitePices;
        if move_obj.side {
            side_y = 0;
            pices = &mut self.BlackPices;
        }
        let rook_new = vec![3, 5];
        let king_new = vec![2, 6];
        let mut king_pos_new = Vec2D::new(king_new[0], side_y);
        let mut rook_pos_new = Vec2D::new(rook_new[0], side_y);
        let mut rook_old: Vec2D = Vec2D::new(0, side_y);
        let king_old: Vec2D = Vec2D::new(4, side_y);
        if move_obj.get_end_pos().y == king_new[1] {
            king_pos_new.x = king_new[1];
            rook_pos_new.x = rook_new[1];
            rook_old.x = 7;
        }
        for p in pices {
            if p.pos.compair(&king_old) && p.TextureID == 0 {
                p.move_pice(&king_pos_new);
            } else if p.pos.compair(&rook_old) && p.TextureID == 2 {
                p.move_pice(&rook_pos_new);
            }
        }
    }

    pub fn transform_pawn(&mut self, side: bool, pice_id: i32) {
        let mut pices = &mut self.BlackPices;
        if !side {
            pices = &mut self.WhitePices;
        }
        for p in pices {
            if p.TextureID == 5 && (p.pos.x == 7 || p.pos.x == 0) {
                p.TextureID = pice_id;
                return;
            }
        }
    }
}