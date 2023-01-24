use macroquad::prelude::*;

const DOOR_ANIMATION_SPEED: i32 = 2;
const WATER_ANIMATION_SPEED: i32 = 8;
const QUESTION_ANIMATION_SPEED: i32 = 8;

// DOORS
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
            if self.update_interval > DOOR_ANIMATION_SPEED {
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

// KEYS
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

// WATER
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
    pub async fn new(room: i32) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 0..=2 {
            let path = format!("assets/images/water/water_{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        let (x, y) = match room {
            -1 => (115.0, 180.0), // for intro
            _ => (0.0, 0.0)
        };

        Self {
            x,
            y,
            texture: sprites,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 28.0,28.0),
            destroyed: false,
            room,
        }
    }

    pub fn update(&mut self) {
        self.update_interval += 1;
        if self.update_interval > WATER_ANIMATION_SPEED {
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

// KEYHOLES
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

// QUESTIONS
pub struct Question {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
    pub destroyed: bool,
    pub room: i32,
}

impl Question {
    pub async fn new(room: i32) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 0..=7 {
            let path = format!("assets/images/question/question_{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        let (x, y) = match room {
            -1 => (115.0, 130.0), // for intro
            _ => (0.0, 0.0)
        };

        Self {
            x,
            y,
            texture: sprites,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 28.0,28.0),
            destroyed: false,
            room,
        }
    }

    pub fn update(&mut self) {
        self.update_interval += 1;
        if self.update_interval > QUESTION_ANIMATION_SPEED {
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