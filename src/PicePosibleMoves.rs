use std::cmp::{max, min};
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use crate::Aox::{Rect2D, Vec2D};
use crate::Board::Board;
use crate::Move::Move;
use crate::Pice::Pice;
use crate::Rendering::draw_rounded_rect_center;

fn get_id_side(id: i32) -> i32 {
    if id == -1 {
        return 0;
    }
    if id < 8 {
        return 1;
    }
    2
}

pub fn pawn_move(pice: &Pice, board: &Board) -> Vec<Vec2D> {
    let mut out: Vec<Vec2D> = vec![];
    if pice.is_taken {
        return out;
    }

    let board_state = board.get_board_state();
    let mut o = 1;
    if pice.side {
        o = -1;
    }
    let front_x = pice.pos.x + o;
    let mut side = 2;
    if !pice.side {
        side = 1;
    }

    let mut s1 = 0;
    let mut s2 = 0;

    if front_x >= 0 && front_x < 8 {
        if pice.pos.y - 1 >= 0 {
            let v1 = board_state[front_x as usize][(pice.pos.y - 1) as usize];
            s1 = get_id_side(v1);
        }
        if pice.pos.y + 1 < 8 {
            let v2 = board_state[front_x as usize][(pice.pos.y + 1) as usize];
            s2 = get_id_side(v2);
        }
    }

    if s1 == side {
        out.push(Vec2D::new(front_x, pice.pos.y - 1))
    }
    if s2 == side {
        out.push(Vec2D::new(front_x, pice.pos.y + 1))
    }
    let mut max_out=1;
    if pice.move_count==0 {
        max_out=2;
    }
    for i in 1..=max_out {
        let x_value = pice.pos.x + i * o;

        if x_value < 0 || x_value >= 8 {
            break;
        }

        if i == 2 {
            if pice.side && pice.pos.x != 6 {
                break;
            }
            if !pice.side && pice.pos.x != 1 {
                break;
            }
        }

        let value = get_id_side(board_state[x_value as usize][pice.pos.y as usize]);
        if value != 0 {
            break;
        } else {
            out.push(Vec2D::new(x_value, pice.pos.y))
        }
    }
    out
}

pub fn rook_moves(pice: &Pice, board: &Board) -> Vec<Vec2D> {
    let mut out: Vec<Vec2D> = vec![];
    if pice.is_taken {
        return out;
    }
    let pos = pice.pos;
    let board_state = board.get_board_state();
    let mut side = 2;
    if !pice.side {
        side = 1;
    }
    let mut arr = vec![false, false, false, false];

    for v in 1..8 {
        let v_i32 = v as i32;

        if !arr[0] {
            if pos.x - v_i32 >= 0 {
                let v1 = get_id_side(board_state[(pos.x - v_i32) as usize][pos.y as usize]);
                if v1 == 0 {
                    out.push(Vec2D::new(pos.x - v_i32, pos.y));
                } else {
                    if v1 == side {
                        out.push(Vec2D::new(pos.x - v_i32, pos.y));
                    }
                    arr[0] = true;
                }
            } else {
                arr[0] = true;
            }
        }

        if !arr[1] {
            if pos.x + v_i32 < 8 {
                let v2 = get_id_side(board_state[(pos.x + v_i32) as usize][pos.y as usize]);
                if v2 == 0 {
                    out.push(Vec2D::new(pos.x + v_i32, pos.y));
                } else {
                    if v2 == side {
                        out.push(Vec2D::new(pos.x + v_i32, pos.y));
                    }
                    arr[1] = true;
                }
            } else {
                arr[1] = true;
            }
        }

        if !arr[2] {
            if pos.y - v_i32 >= 0 {
                let v3 = get_id_side(board_state[pos.x as usize][(pos.y - v_i32) as usize]);
                if v3 == 0 {
                    out.push(Vec2D::new(pos.x, pos.y - v_i32));
                } else {
                    if v3 == side {
                        out.push(Vec2D::new(pos.x, pos.y - v_i32));
                    }
                    arr[2] = true;
                }
            } else {
                arr[2] = true;
            }
        }

        if !arr[3] {
            if pos.y + v_i32 < 8 {
                let v4 = get_id_side(board_state[pos.x as usize][(pos.y + v_i32) as usize]);
                if v4 == 0 {
                    out.push(Vec2D::new(pos.x, pos.y + v_i32));
                } else {
                    if v4 == side {
                        out.push(Vec2D::new(pos.x, pos.y + v_i32));
                    }
                    arr[3] = true;
                }
            } else {
                arr[3] = true;
            }
        }
    }
    return out
}

pub fn bishop_moves(pice: &Pice, board: &Board) -> Vec<Vec2D> {
    let mut out: Vec<Vec2D> = vec![];
    if pice.is_taken {
        return out;
    }
    let pos = pice.pos;
    let board_state = board.get_board_state();
    let mut side = 2;
    if !pice.side {
        side = 1;
    }
    let mut arr = vec![false, false, false, false];

    for v in 1..8 {
        let v_i32 = v as i32;

        if !arr[0] {
            if pos.x - v_i32 >= 0 && pos.y - v_i32 >= 0 {
                let v1 = get_id_side(board_state[(pos.x - v_i32) as usize][(pos.y - v_i32) as usize]);
                if v1 == 0 {
                    out.push(Vec2D::new(pos.x - v_i32, pos.y - v_i32));
                } else {
                    if v1 == side {
                        out.push(Vec2D::new(pos.x - v_i32, pos.y - v_i32));
                    }
                    arr[0] = true;
                }
            } else {
                arr[0] = true;
            }
        }

        if !arr[1] {
            if pos.x - v_i32 >= 0 && pos.y + v_i32 < 8 {
                let v2 = get_id_side(board_state[(pos.x - v_i32) as usize][(pos.y + v_i32) as usize]);
                if v2 == 0 {
                    out.push(Vec2D::new(pos.x - v_i32, pos.y + v_i32));
                } else {
                    if v2 == side {
                        out.push(Vec2D::new(pos.x - v_i32, pos.y + v_i32));
                    }
                    arr[1] = true;
                }
            } else {
                arr[1] = true;
            }
        }

        if !arr[2] {
            if pos.x + v_i32 < 8 && pos.y - v_i32 >= 0 {
                let v3 = get_id_side(board_state[(pos.x + v_i32) as usize][(pos.y - v_i32) as usize]);
                if v3 == 0 {
                    out.push(Vec2D::new(pos.x + v_i32, pos.y - v_i32));
                } else {
                    if v3 == side {
                        out.push(Vec2D::new(pos.x + v_i32, pos.y - v_i32));
                    }
                    arr[2] = true;
                }
            } else {
                arr[2] = true;
            }
        }

        if !arr[3] {
            if pos.x + v_i32 < 8 && pos.y + v_i32 < 8 {
                let v4 = get_id_side(board_state[(pos.x + v_i32) as usize][(pos.y + v_i32) as usize]);
                if v4 == 0 {
                    out.push(Vec2D::new(pos.x + v_i32, pos.y + v_i32));
                } else {
                    if v4 == side {
                        out.push(Vec2D::new(pos.x + v_i32, pos.y + v_i32));
                    }
                    arr[3] = true;
                }
            } else {
                arr[3] = true;
            }
        }
    }
    out
}

pub fn queen_moves(pice: &Pice, board: &Board) -> Vec<Vec2D> {
    let mut out: Vec<Vec2D> = bishop_moves(pice, board);
    if pice.is_taken {
        return out;
    }
    out.append(&mut rook_moves(pice, board));
    return out;
}

pub fn knight_moves(pice: &Pice, board: &Board) -> Vec<Vec2D> {
    let mut out: Vec<Vec2D> = vec![];
    if pice.is_taken {
        return out;
    }
    let pos = pice.pos;
    let board_state = board.get_board_state();
    let mut side = 2;
    if !pice.side {
        side = 1;
    }

    let moves = [
        (pos.x - 2, pos.y - 1),
        (pos.x - 2, pos.y + 1),
        (pos.x - 1, pos.y - 2),
        (pos.x - 1, pos.y + 2),
        (pos.x + 1, pos.y - 2),
        (pos.x + 1, pos.y + 2),
        (pos.x + 2, pos.y - 1),
        (pos.x + 2, pos.y + 1),
    ];

    for (x, y) in moves {
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            let v = get_id_side(board_state[x as usize][y as usize]);
            if v == 0 || v == side {
                out.push(Vec2D::new(x, y));
            }
        }
    }
    return out
}

pub fn king_moves(pice: &Pice, board: &Board) -> Vec<Vec2D> {
    let mut out: Vec<Vec2D> = vec![];
    if pice.is_taken {
        return out;
    }
    let pos = pice.pos;
    let board_state = board.get_board_state();
    let mut side = 2;
    if !pice.side {
        side = 1;
    }

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let nx = pos.x + i;
            let ny = pos.y + j;

            if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 {
                let v = get_id_side(board_state[nx as usize][ny as usize]);
                if v == 0 || v == side {
                    out.push(Vec2D::new(nx, ny));
                }
            }
        }
    }
    out
}

pub fn get_casteling_moves(pice: &Pice, board: &Board) -> Vec<Vec2D> {
    let mut out: Vec<Vec2D> = Vec::new();
    let row_x = if pice.side { 7 } else { 0 };
    if pice.TextureID != 0 || pice.move_count > 0 || pice.is_taken || pice.pos.x != row_x || pice.pos.y != 4 {
        return out;
    }
    let my_pieces = if pice.side { &board.WhitePices } else { &board.BlackPices };

    let mut can_castle_short = false;
    let mut can_castle_long = false;
    for p in my_pieces {
        if p.TextureID == 2 && p.move_count == 0 && !p.is_taken && p.pos.x == row_x {
            if p.pos.y == 7 { can_castle_short = true; }
            if p.pos.y == 0 { can_castle_long = true; }
        }
    }
    let board_state = board.get_board_state();
    if can_castle_short {
        if board_state[row_x as usize][5] != -1 || board_state[row_x as usize][6] != -1 {
            can_castle_short = false;
        }
    }
    if can_castle_long {
        if board_state[row_x as usize][1] != -1 || board_state[row_x as usize][2] != -1 || board_state[row_x as usize][3] != -1 {
            can_castle_long = false;
        }
    }
    if can_castle_short || can_castle_long {
        let opponent_pieces = if pice.side { &board.BlackPices } else { &board.WhitePices };
        let mut attacked_squares = vec![vec![false; 8]; 8];

        for p in opponent_pieces {
            if p.is_taken { continue; }

            if p.TextureID == 5 {
                let direction = if p.side { -1 } else { 1 };
                let attack_x = p.pos.x + direction;
                if attack_x >= 0 && attack_x < 8 {
                    if p.pos.y - 1 >= 0 { attacked_squares[attack_x as usize][(p.pos.y - 1) as usize] = true; }
                    if p.pos.y + 1 < 8 { attacked_squares[attack_x as usize][(p.pos.y + 1) as usize] = true; }
                }
            } else {
                let basic_moves = match p.TextureID {
                    0 => king_moves(p, board),
                    1 => queen_moves(p, board),
                    2 => rook_moves(p, board),
                    3 => bishop_moves(p, board),
                    4 => knight_moves(p, board),
                    _ => vec![],
                };
                for m in basic_moves {
                    attacked_squares[m.x as usize][m.y as usize] = true;
                }
            }
        }

        if attacked_squares[row_x as usize][4] {
            can_castle_short = false;
            can_castle_long = false;
        }

        if can_castle_short {
            if attacked_squares[row_x as usize][5] || attacked_squares[row_x as usize][6] {
                can_castle_short = false;
            }
        }
        if can_castle_long {
            if attacked_squares[row_x as usize][3] || attacked_squares[row_x as usize][2] {
                can_castle_long = false;
            }
        }
    }

    if can_castle_short {
        out.push(Vec2D::new(row_x, 6));
    }
    if can_castle_long {
        out.push(Vec2D::new(row_x, 2));
    }

    out
}

#[derive(Clone)]
pub struct PosibleMoves {
    pub moves: Vec<Move>,
    pub opacity: f32,
}

impl PosibleMoves {
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            opacity: 0.0,
        }
    }

    pub fn compute_moves(&mut self, pice: &Pice, board: &Board, check_safety: bool) {
        self.clear();
        self.opacity = 0.0;
        let calculated_moves = match pice.TextureID {
            0 => king_moves(pice, board),
            1 => queen_moves(pice, board),
            2 => rook_moves(pice, board),
            3 => bishop_moves(pice, board),
            4 => knight_moves(pice, board),
            5 => pawn_move(pice, board),
            _ => vec![],
        };
        let side = pice.side;
        self.moves = Vec::new();
        for m in calculated_moves {
            let move_item = Move::from_pos(pice.pos.clone(), m.clone(), pice.TextureID, pice.side, false);
            if check_safety {
                if !board.is_move_resoult_in_check(&move_item, side) {
                    self.moves.push(move_item);
                }
            } else {
                self.moves.push(move_item);
            }
        }

        if pice.TextureID == 0 {
            let casteling_move = get_casteling_moves(pice, board);
            for m in &casteling_move {
                let move_item = Move::from_pos(pice.pos.clone(), m.clone(), pice.TextureID, pice.side, true);

                if check_safety {
                    if !board.is_move_resoult_in_check(&move_item, side) {
                        self.moves.push(move_item);
                    }
                } else {
                    self.moves.push(move_item);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.moves = Vec::new();
    }

    fn update_animation(&mut self, _d: &mut RaylibDrawHandle, frame_time: f32) {
        let is_active = self.moves.len() > 0;
        let speed = 5.0;
        if is_active {
            self.opacity += speed * frame_time;
        } else {
            self.opacity -= speed * frame_time;
        }
        if self.opacity < 0.0 {
            self.opacity = 0.0;
        } else if self.opacity > 1.0 {
            self.opacity = 1.0;
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, positions: &Vec<Vec<Vec2D>>) {
        for move_pos in &self.moves {
            let end = move_pos.get_end_pos();
            let pos = positions[end.x as usize][end.y as usize];
            let size = 100.0;
            let roundes = 0.2;
            let color = Color::new(255, 0, 0, (255.0 * self.opacity) as u8);
            draw_rounded_rect_center(d, size, size, roundes, pos, color,1.0)
        }
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        self.update_animation(d, d.get_frame_time());
    }
}