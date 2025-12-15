#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2D {
    pub x: i32,
    pub y: i32,
}

impl Vec2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn is_null(&self)->bool{
        return self.x==-1&&self.y==-1;
    }
    pub fn compair(&self,other:&Vec2D)->bool{
        return self.x==other.x&&self.y==other.y;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect2D {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect2D {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn get_center(&self) -> Vec2D {
        Vec2D::new(self.x + (self.width / 2), self.y + (self.height / 2))
    }
    pub fn contains(&self, point: Vec2D) -> bool {
        point.x >= self.x && point.x <= self.x + self.width && point.y >= self.y && point.y <= self.y + self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line2D {
    pub start: Vec2D,
    pub end: Vec2D,
}

impl Line2D {
    pub fn new(start: Vec2D, end: Vec2D) -> Self {
        Self { start, end }
    }
    pub fn point_at(&self, t: f32) -> Vec2D {
        let x = (self.start.x as f32 + (self.end.x - self.start.x) as f32 * t).round() as i32;
        let y = (self.start.y as f32 + (self.end.y - self.start.y) as f32 * t).round() as i32;
        Vec2D::new(x, y)
    }
}

pub fn get_board_draw_positions(start_x: i32, start_y: i32, board_size: i32) -> Vec<Vec<Vec2D>> {
    let tile_size = board_size / 8;
    let mut centers = Vec::with_capacity(8);

    for row in 0..8 {
        let mut row_centers = Vec::with_capacity(8);
        for col in 0..8 {
            let x = start_x + (col * tile_size);
            let y = start_y + (row * tile_size);

            let rect = Rect2D::new(x, y, tile_size, tile_size);
            row_centers.push(rect.get_center());
        }
        centers.push(row_centers);
    }
    centers
}

pub fn get_click_rect(start_x: i32, start_y: i32, board_size: i32) -> Vec<Vec<Rect2D>> {
    let tile_size = board_size / 8;
    let mut out: Vec<Vec<Rect2D>> = Vec::new();
    for row in 0..8 {
        let mut row_vec: Vec<Rect2D> = Vec::new();
        for col in 0..8 {
            let x = start_x + (col * tile_size);
            let y = start_y + (row * tile_size);
            row_vec.push(Rect2D::new(x, y, tile_size, tile_size));
        }
        out.push(row_vec);
    }
    out
}

pub fn swap_on_matrix(input: &mut Vec<Vec<i32>>, p1: Vec2D, p2: Vec2D) {
    if p1.x < 0 || p2.x < 0 || p1.y < 0 || p2.y < 0 {
        return;
    }

    let p1_x = p1.x as usize;
    let p1_y = p1.y as usize;
    let p2_x = p2.x as usize;
    let p2_y = p2.y as usize;

    if p1_x >= input.len() || p2_x >= input.len() {
        return;
    }

    if p1_y >= input[p1_x].len() || p2_y >= input[p2_x].len() {
        return;
    }

    let v = input[p1_x][p1_y];
    input[p1_x][p1_y] = input[p2_x][p2_y];
    input[p2_x][p2_y] = v;
}