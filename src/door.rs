use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 2;

pub struct Door {
    pub x: f32,
    pub y: f32,
    pub texture: Vec<Texture2D>,
    pub rect: Rect,
    pub destroyed: bool,
    pub room: i32,
    animation_completed: bool,
    update_interval: i32,
    cur_frame: usize,
    pub show_open_animation: bool,
}

impl Door {
    pub async fn new(room: i32) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();
        let mut x: f32 = 0.0;
        let mut level: i32 = 0;

        match room {
            15 => {
                x = 0.0;
                level = 1;
            },
            37 => {
                x = 1152.0;
                level = 1;
            },
            40 => {
                x = 1152.0;
                level = 2;
            },
            47 => {
                x = 0.0;
                level = 2;
            },
            55 => {
                x = 1152.0;
                level = 2;
            },
            92 => {
                x = 1152.0;
                level = 3;
            },
            _ => {}
        };

        for i in 0..=36 {
            let path = format!("assets/images/doors/level_{}/door_{}.png", level, i);
            sprites.push(load_texture(&path).await.unwrap());
        }
        
        Self {
            x,
            y: 288.0,
            texture: sprites,
            rect: Rect::new(0.0, 0.0, 24.0, 24.0),
            room,
            destroyed: false,
            animation_completed: false,
            update_interval: 0,
            cur_frame: 0,
            show_open_animation: false,
        }
    }

    pub fn open_animation(&mut self) {
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

    pub fn update(&mut self) {
        self.rect.w = self.texture[0].width();
        self.rect.h = self.texture[0].height();
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        self.update();
        if !self.animation_completed && self.show_open_animation {
            self.open_animation();
        } else {
            draw_texture(self.texture[0], self.x, self.y, WHITE);
        }
    }
}
