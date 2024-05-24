use rand::Rng;

pub enum GoldPosition {
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

impl GoldPosition {
    pub fn rand_gold_position() -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..=3)
    }
}
