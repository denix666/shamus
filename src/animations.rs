use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 5;

pub struct PlayerDieAnimation {
    pub x: f32,
    pub y: f32,
    pub texture: Vec<Texture2D>,
    pub destroyed: bool,
    animation_completed: bool,
    update_interval: i32,
    cur_frame: usize,
}

impl PlayerDieAnimation {
    pub async fn new(x: f32, y: f32) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 0..=2 {
            let path = format!("assets/images/player/die_{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }
        
        Self {
            x,
            y,
            texture: sprites,
            destroyed: false,
            animation_completed: false,
            update_interval: 0,
            cur_frame: 0,
        }
    }

    pub fn show_animation(&mut self) {
        if !self.animation_completed {
            draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame += 1;
                if self.cur_frame == self.texture.len() {
                    self.cur_frame = 0;
                    self.animation_completed = true;
                    self.destroyed = true;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        if !self.animation_completed {
            self.show_animation();
        } else {
            draw_texture(self.texture[0], self.x, self.y, WHITE);
        }
    }
}


pub struct EnemyDieAnimation {
    pub x: f32,
    pub y: f32,
    pub texture: Vec<Texture2D>,
    pub destroyed: bool,
    animation_completed: bool,
    update_interval: i32,
    cur_frame: usize,
}

impl EnemyDieAnimation {
    pub async fn new(x: f32, y: f32) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 0..=4 {
            let path = format!("assets/images/enemy/die_{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }
        
        Self {
            x,
            y,
            texture: sprites,
            destroyed: false,
            animation_completed: false,
            update_interval: 0,
            cur_frame: 0,
        }
    }

    pub fn show_animation(&mut self) {
        if !self.animation_completed {
            draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame += 1;
                if self.cur_frame == self.texture.len() {
                    self.cur_frame = 0;
                    self.animation_completed = true;
                    self.destroyed = true;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        if !self.animation_completed {
            self.show_animation();
        } else {
            draw_texture(self.texture[0], self.x, self.y, WHITE);
        }
    }
}