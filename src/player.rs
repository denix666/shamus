use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 3;
const PLAYER_SPEED: f32 = 200.0;

#[derive(PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Idle,
}

pub struct Player {
    pub x: f32,
    pub y: f32, 
    up_textures: Vec<Texture2D>,
    down_textures: Vec<Texture2D>,
    left_textures: Vec<Texture2D>,
    right_textures: Vec<Texture2D>,
    idle_texture: Texture2D,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
    pub dir: Dir,
}

impl Player {
    pub async fn new()  -> Self {
        let mut up_sprites:Vec<Texture2D> = Vec::new();
        for i in 0..=2 {
            let path = format!("assets/images/player/up_{}.png", i);
            up_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut down_sprites:Vec<Texture2D> = Vec::new();
        for i in 0..=2 {
            let path = format!("assets/images/player/down_{}.png", i);
            down_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut left_sprites:Vec<Texture2D> = Vec::new();
        for i in 0..=2 {
            let path = format!("assets/images/player/left_{}.png", i);
            left_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut right_sprites:Vec<Texture2D> = Vec::new();
        for i in 0..=2 {
            let path = format!("assets/images/player/right_{}.png", i);
            right_sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x: crate::resources::PLAYER_START_X_POS,
            y: crate::resources::PLAYER_START_Y_POS,
            up_textures: up_sprites,
            down_textures: down_sprites,
            left_textures: left_sprites,
            right_textures: right_sprites,
            idle_texture: load_texture("assets/images/player/idle.png").await.unwrap(),
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 28.0, 24.0),
            dir: Dir::Idle,
        }
    }

    pub fn draw_lives(&mut self, lives: i32) {
        let lives_to_show: i32;
        if lives >= 13 {
            lives_to_show = 12;
        } else {
            lives_to_show = lives;
        }

        let mut ly: i32 = 0;
        let mut lx: i32;
        for i in 0..lives_to_show {
            lx = 1081 + 30 * i;
            if i > 3 && i < 8 {
                ly = 30;
                lx = 1081 + 30 * (i - 4);
            } else {
                if i > 7 {
                    ly = 60;
                    lx = 1081 + 30 * (i - 8);
                }
            }
            draw_texture(self.idle_texture, lx as f32, ly as f32, WHITE);
        }
    }

    pub fn update(&mut self, dt: f32) {
        let (v_mv, h_mv, dir): (f32, f32, Dir) = match (
            is_key_down(KeyCode::Down),
            is_key_down(KeyCode::Up),
            is_key_down(KeyCode::Right),
            is_key_down(KeyCode::Left),
        ) {
            (true, false, false, false) => (1.0, 0.0, Dir::Down),
            (false, true, false, false) => (-1.0, 0.0, Dir::Up),
            (false, false, true, false) => (0.0, 1.0, Dir::Right),
            (false, false, false, true) => (0.0, -1.0, Dir::Left),
            (false, true, false, true) => (-1.0, -1.0, Dir::UpLeft),
            (true, false, false, true) => (1.0, -1.0, Dir::DownLeft),
            (false, true, true, false) => (-1.0, 1.0, Dir::UpRight),
            (true, false, true, false) => (1.0, 1.0, Dir::DownRight),
            _ => (0.0, 0.0, Dir::Idle),
        };
    
        self.x += dt * PLAYER_SPEED * h_mv;
        self.y += dt * PLAYER_SPEED * v_mv;
        self.dir = dir;

        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.up_textures.len() {
                self.cur_frame = 0;
            }
        }

        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        match self.dir {
            Dir::Up => {draw_texture(self.up_textures[self.cur_frame], self.x, self.y, WHITE);},
            Dir::Down => {draw_texture(self.down_textures[self.cur_frame], self.x, self.y, WHITE);},
            Dir::Left => {draw_texture(self.left_textures[self.cur_frame], self.x, self.y, WHITE);},
            Dir::Right => {draw_texture(self.right_textures[self.cur_frame], self.x, self.y, WHITE);},
            Dir::Idle => {draw_texture(self.idle_texture, self.x, self.y, WHITE);},
            Dir::UpLeft => {draw_texture(self.left_textures[self.cur_frame], self.x, self.y, WHITE);},
            Dir::DownLeft => {draw_texture(self.left_textures[self.cur_frame], self.x, self.y, WHITE);},
            Dir::UpRight => {draw_texture(self.right_textures[self.cur_frame], self.x, self.y, WHITE);},
            Dir::DownRight => {draw_texture(self.right_textures[self.cur_frame], self.x, self.y, WHITE);},
        }
    }
}
