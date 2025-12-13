#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2D {
    pub x: i32,
    pub y: i32,
}

impl Vec2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
    
}
