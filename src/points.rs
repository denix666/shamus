use macroquad::prelude::Rect;

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub value: String,
    pub rect: Rect,
}

impl Point {
    pub fn new(x: f32, y: f32, value: String) -> Self {
        Self {
            x,
            y,
            value,
            rect: Rect::new(x, y, 24.0, 24.0),
        }
    }
}
