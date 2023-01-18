use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 8;

pub struct Water {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
    pub destroyed: bool,
    pub room: i32,
}

impl Water {
    pub async fn new() -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 0..=2 {
            let path = format!("assets/images/water/water_{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x: 0.0,
            y: 0.0,
            texture: sprites,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 28.0,28.0),
            destroyed: false,
            room: 0,
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

        self.rect.w = self.texture[self.cur_frame].width();
        self.rect.h = self.texture[self.cur_frame].height();
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        self.update();
        draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);
    }
}
