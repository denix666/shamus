pub struct RoomProperty {
    pub room_to: i32,
    pub water: bool,
    pub question: bool,
    pub key: bool,
    pub key_color: String,
    pub keyhole: bool,
    pub keyhole_color: String,
}

impl RoomProperty {
    pub fn new() -> Self {
        Self {
            room_to: 0,
            water: false,
            question: false,
            key: false,
            key_color: "".to_string(),
            keyhole: false,
            keyhole_color: "".to_string(),
        }
    }
}