use crate::Aox::Vec2D;
use crate::Board::Board;
use crate::Pice::Pice;

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

    for i in 1..=2 {
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
                if v1 != side {
                    if v1 != 0 {
                        arr[0] = true;
                    }
                    out.push(Vec2D::new(pos.x - v_i32, pos.y));
                } else {
                    arr[0] = true;
                }
            } else {
                arr[0] = true;
            }
        }

        if !arr[1] {
            if pos.x + v_i32 < 8 {
                let v2 = get_id_side(board_state[(pos.x + v_i32) as usize][pos.y as usize]);
                if v2 != side {
                    if v2 != 0 {
                        arr[1] = true;
                    }
                    out.push(Vec2D::new(pos.x + v_i32, pos.y));
                } else {
                    arr[1] = true;
                }
            } else {
                arr[1] = true;
            }
        }

        if !arr[2] {
            if pos.y - v_i32 >= 0 {
                let v3 = get_id_side(board_state[pos.x as usize][(pos.y - v_i32) as usize]);
                if v3 != side {
                    if v3 != 0 {
                        arr[2] = true;
                    }
                    out.push(Vec2D::new(pos.x, pos.y - v_i32));
                } else {
                    arr[2] = true;
                }
            } else {
                arr[2] = true;
            }
        }

        if !arr[3] {
            if pos.y + v_i32 < 8 {
                let v4 = get_id_side(board_state[pos.x as usize][(pos.y + v_i32) as usize]);
                if v4 != side {
                    if v4 != 0 {
                        arr[3] = true;
                    }
                    out.push(Vec2D::new(pos.x, pos.y + v_i32));
                } else {
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
                if v1 != side {
                    if v1 != 0 {
                        arr[0] = true;
                    }
                    out.push(Vec2D::new(pos.x - v_i32, pos.y - v_i32));
                } else {
                    arr[0] = true;
                }
            } else {
                arr[0] = true;
            }
        }

        if !arr[1] {
            if pos.x - v_i32 >= 0 && pos.y + v_i32 < 8 {
                let v2 = get_id_side(board_state[(pos.x - v_i32) as usize][(pos.y + v_i32) as usize]);
                if v2 != side {
                    if v2 != 0 {
                        arr[1] = true;
                    }
                    out.push(Vec2D::new(pos.x - v_i32, pos.y + v_i32));
                } else {
                    arr[1] = true;
                }
            } else {
                arr[1] = true;
            }
        }

        if !arr[2] {
            if pos.x + v_i32 < 8 && pos.y - v_i32 >= 0 {
                let v3 = get_id_side(board_state[(pos.x + v_i32) as usize][(pos.y - v_i32) as usize]);
                if v3 != side {
                    if v3 != 0 {
                        arr[2] = true;
                    }
                    out.push(Vec2D::new(pos.x + v_i32, pos.y - v_i32));
                } else {
                    arr[2] = true;
                }
            } else {
                arr[2] = true;
            }
        }

        if !arr[3] {
            if pos.x + v_i32 < 8 && pos.y + v_i32 < 8 {
                let v4 = get_id_side(board_state[(pos.x + v_i32) as usize][(pos.y + v_i32) as usize]);
                if v4 != side {
                    if v4 != 0 {
                        arr[3] = true;
                    }
                    out.push(Vec2D::new(pos.x + v_i32, pos.y + v_i32));
                } else {
                    arr[3] = true;
                }
            } else {
                arr[3] = true;
            }
        }
    }
    out
}

pub fn queen_moves(pice: Pice, board: &Board) -> Vec<Vec2D> {
    let mut out: Vec<Vec2D> = bishop_moves(&pice, board);
    out.append(&mut rook_moves(&pice, board));
    return out;
}

pub fn knight_moves(pice: Pice, board: &Board) -> Vec<Vec2D> {
    let mut out: Vec<Vec2D> = vec![];
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
            if v != side {
                out.push(Vec2D::new(x, y));
            }
        }
    }
    return out
}