pub struct Player {
    pub x: f32,
    pub y: f32,
    pub score: i16,
    pub steps: i16,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            x,
            y,
            score: 0,
            steps: 0,
        }
    }
}
