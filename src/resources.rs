use macroquad::prelude::*;

pub const RES_WIDTH: i32 = 1200;
pub const RES_HEIGHT: i32 = 720;
//pub const TILE_SIZE: f32 = 24.0;

pub const PLAYER_START_X_POS: f32 = 100.0;
pub const PLAYER_START_Y_POS: f32 = 350.0;

pub const ENEMY_SPEED: f32 = 100.0;

// How match time player can be in the room before the shadow get in
pub const MAX_TIME_IN_THE_ROOM: f64 = 12.0;
pub const SHADOW_SPEED: f32 = 120.0;
pub const SHADOW_FREEZE_TIME: f64 = 2.0;

pub struct Resources {
    pub font: Font,
    pub bg_a: Texture2D,
    pub bg_b: Texture2D,
    pub bg_c: Texture2D,
    pub bg_d: Texture2D,
    pub bg_e: Texture2D,
    pub bg_f: Texture2D,
    pub bg_g: Texture2D,
    pub bg_i: Texture2D,
    pub bg_j: Texture2D,
    pub bg_k: Texture2D,
    pub bg_r: Texture2D,
    pub bg_u: Texture2D,
    pub intro: Texture2D,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            font: load_ttf_font("assets/fonts/game_font.ttf").await.unwrap(),
            bg_a: load_texture("assets/textures/a.png").await.unwrap(),
            bg_b: load_texture("assets/textures/b.png").await.unwrap(),
            bg_c: load_texture("assets/textures/c.png").await.unwrap(),
            bg_d: load_texture("assets/textures/d.png").await.unwrap(),
            bg_e: load_texture("assets/textures/e.png").await.unwrap(),
            bg_f: load_texture("assets/textures/f.png").await.unwrap(),
            bg_g: load_texture("assets/textures/g.png").await.unwrap(),
            bg_i: load_texture("assets/textures/i.png").await.unwrap(),
            bg_j: load_texture("assets/textures/j.png").await.unwrap(),
            bg_k: load_texture("assets/textures/k.png").await.unwrap(),
            bg_r: load_texture("assets/textures/r.png").await.unwrap(),
            bg_u: load_texture("assets/textures/u.png").await.unwrap(),
            intro: load_texture("assets/images/intro.png").await.unwrap(),
        }
    }
}