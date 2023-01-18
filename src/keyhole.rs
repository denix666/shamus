use macroquad::prelude::*;

pub struct KeyHole {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    pub rect: Rect,
    pub keyhole_color: String,
    pub room: i32,
    pub destroyed: bool,
}

impl KeyHole {
    pub async fn new() -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();
        let key_colors=["red", "purple", "green", "gold", "cyan", "blue"];

        for i in key_colors.iter() {
            let path = format!("assets/images/keyholes/{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }
        
        Self {
            x: 0.0,
            y: 0.0,
            texture: sprites,
            rect: Rect::new(0.0, 0.0, 128.0, 128.0),
            keyhole_color: "".to_string(),
            room: 0,
            destroyed: false,
        }
    }

    pub fn update(&mut self) {
        self.rect.w = self.texture[0].width();
        self.rect.h = self.texture[0].height();
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        self.update();
        match self.keyhole_color.as_str() {
            "red" => {draw_texture(self.texture[0], self.x, self.y, WHITE)},
            "purple" => {draw_texture(self.texture[1], self.x, self.y, WHITE)},
            "green" => {draw_texture(self.texture[2], self.x, self.y, WHITE)},
            "gold" => {draw_texture(self.texture[3], self.x, self.y, WHITE)},
            "cyan" => {draw_texture(self.texture[4], self.x, self.y, WHITE)},
            "blue" => {draw_texture(self.texture[5], self.x, self.y, WHITE)},
            _ => {}
        }
        
    }
}
