pub struct Game {
    pub level: i32,
    pub score: i32,
    pub lives: i32,
    pub room: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            level: 0,
            score: 0,
            lives: 0,
            room: 0,
        }
    }
}
