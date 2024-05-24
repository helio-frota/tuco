pub struct Player {
    pub x: f64,
    pub y: f64,
    pub score: i16,
    pub steps: i16,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        Player {
            x: x / 2.0,
            y: y / 2.0,
            score: 0,
            steps: 0,
        }
    }
}
