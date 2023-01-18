use macroquad::prelude::*;

pub struct KeyHole {
    pub x: f32,
    pub y: f32,
    texture: Texture2D,
    pub rect: Rect,
    pub keyhole_color: String,
    pub room: i32,
    pub destroyed: bool,
}

impl KeyHole {
    pub async fn new(room: i32, keyhole_color: &str) -> Self {
        let path = format!("assets/images/keyholes/{}.png", keyhole_color);
        
        Self {
            x: 0.0,
            y: 0.0,
            texture: load_texture(&path).await.unwrap(),
            rect: Rect::new(0.0, 0.0, 28.0, 28.0),
            keyhole_color: keyhole_color.to_string(),
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
