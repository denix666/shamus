use macroquad::prelude::*;

pub struct Key {
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
    pub rect: Rect,
    pub key_color: String,
    pub destroyed: bool,
    pub room: i32,
}

impl Key {
    pub async fn new(room: i32, key_color: &str) -> Self {
        let path = format!("assets/images/keys/{}.png", key_color);
        
        Self {
            x: 0.0,
            y: 0.0,
            texture: load_texture(&path).await.unwrap(),
            rect: Rect::new(0.0, 0.0, 42.0, 42.0),
            key_color: key_color.to_string(),
            room,
            destroyed: false,
        }
    }

    pub fn update(&mut self) {
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        self.update();
        draw_texture(self.texture, self.x, self.y, WHITE);
    }
}
