use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 12;

pub struct Shadow {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
    pub freeze: bool,
    pub freeze_time: f64,
}

impl Shadow {
    pub async fn new(x:f32, y:f32) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 0..=2 {
            let path = format!("assets/images/enemy/shadow_{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x,
            y,
            texture: sprites,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 28.0,28.0),
            freeze: false,
            freeze_time: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.texture.len() {
                self.cur_frame = 0;
            }
        }
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);
    }
}
