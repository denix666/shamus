use macroquad::prelude::*;

const ENEMY_ANIMATION_SPEED: i32 = 8;
const SHADOW_ANIMATION_SPEED: i32 = 8;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
    pub destroyed: bool,
}

impl Enemy {
    pub async fn new(x:f32, y:f32, enemy_type: &str) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 0..=2 {
            let path = format!("assets/images/enemy/{}_{}.png", enemy_type, i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x,
            y,
            texture: sprites,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 28.0,28.0),
            destroyed: false,
        }
    }

    pub fn update(&mut self, points: &Vec<crate::Point>, prev_x: f32, prev_y: f32) {
        self.update_interval += 1;
        if self.update_interval > ENEMY_ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.texture.len() {
                self.cur_frame = 0;
            }
        }

        self.rect.x = self.x;
        self.rect.y = self.y;

        for point in points {
            if let Some(_i) = self.rect.intersect(point.rect) {
                self.x = prev_x;
                self.y = prev_y;
                break;
            }
        }
    }

    pub fn draw(&mut self) {
        draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);
    }
}

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
        if self.update_interval > SHADOW_ANIMATION_SPEED {
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