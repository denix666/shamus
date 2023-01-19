use macroquad::prelude::*;

const BULLET_SPEED: f32 = 400.0;

pub enum PlayerBulletDir {
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

pub struct PlayerBullet {
    x: f32,
    y: f32,
    texture: Texture2D,
    pub destroyed: bool,
    pub rect: Rect,
    pub dir: PlayerBulletDir,
    angel: f32,
}

impl PlayerBullet {
    pub async fn new(x:f32, y:f32, dir: PlayerBulletDir) -> Self {
        Self { 
            x,
            y,
            dir,
            texture: load_texture("assets/images/bullet.png").await.unwrap(),
            destroyed: false,
            rect: Rect::new(x, y, 3.0, 3.0),
            angel: 0.0,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let (x, y, angel, speed): (f32, f32, f32, f32) = match self.dir {
            PlayerBulletDir::Up => (0.0, -1.0, 4.71239, BULLET_SPEED),
            PlayerBulletDir::Down => (0.0, 1.0, 1.5708, BULLET_SPEED),
            PlayerBulletDir::Left => (-1.0, 0.0, 3.14159, BULLET_SPEED),
            PlayerBulletDir::Right => (1.0, 0.0, 0.0, BULLET_SPEED),
            PlayerBulletDir::UpLeft => (-1.0, -1.0, 3.92699, BULLET_SPEED / 2.0),
            PlayerBulletDir::UpRight => (1.0, -1.0, 5.49779, BULLET_SPEED / 2.0),
            PlayerBulletDir::DownLeft => (-1.0, 1.0, 2.35619, BULLET_SPEED / 2.0),
            PlayerBulletDir::DownRight => (1.0, 1.0, 0.785398, BULLET_SPEED / 2.0),
            PlayerBulletDir::Idle => (0.0, 0.0, 0.0, BULLET_SPEED),
        };
    
        self.x += dt * speed * x;
        self.y += dt * speed * y;
        self.angel = angel;
        
        if self.y < 0.0 || self.y > screen_height() || self.x < 0.0 || self.x > screen_width() {
            self.destroyed = true;
        }
    }

    pub fn draw(&mut self) {
        if !self.destroyed {
            self.rect.x = self.x;
            self.rect.y = self.y;

            self.update_position(get_frame_time());
            
            draw_texture_ex(self.texture, self.x, self.y, WHITE, 
                DrawTextureParams {
                    rotation: self.angel,
                    ..Default::default()
                },
            );
        }
    }
}