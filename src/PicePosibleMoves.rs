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

pub fn pawn_move(pice: Pice, board: &Board) -> Vec<Vec2D> {
    let mut out: Vec<Vec2D> = vec![];
    let board_state = board.get_board_state();

    let direction = if pice.side { -1 } else { 1 };
    let start_x = if pice.side { 6 } else { 1 };

    let current_x = pice.pos.x;
    let current_y = pice.pos.y;

    let next_x = current_x + direction;

    if next_x >= 0 && next_x < 8 {
        if board_state[next_x as usize][current_y as usize] == -1 {
            out.push(Vec2D::new(next_x, current_y));

            if current_x == start_x {
                let double_x = current_x + (direction * 2);
                if double_x >= 0 && double_x < 8 {
                    if board_state[double_x as usize][current_y as usize] == -1 {
                        out.push(Vec2D::new(double_x, current_y));
                    }
                }
            }
        }

        let capture_ys = [current_y - 1, current_y + 1];
        for &cap_y in &capture_ys {
            if cap_y >= 0 && cap_y < 8 {
                let target_id = board_state[next_x as usize][cap_y as usize];
                let target_side = get_id_side(target_id);
                let my_side = if pice.side { 1 } else { 2 };

                if target_id != -1 && target_side != my_side {
                    out.push(Vec2D::new(next_x, cap_y));
                }
            }
        }
    }

  return out
}