pub struct Player {
    pub x: f64,
    pub y: f64,
    pub score: i16,
    pub steps: i16,
}

impl Player {
    pub fn new(display_width: f64, display_height: f64) -> Player {
        Player {
            x: display_width / 2.0,
            y: display_height / 2.0,
            score: 0,
            steps: 0,
        }
    }
}
