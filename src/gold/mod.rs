use rand::Rng;
pub enum GoldPosition {
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

impl GoldPosition {
    /**
     * This function is used only to return a random number between 0 and 3
     * to be used as a gold position.
     */
    pub fn rand_gold_position() -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..=3)
    }
}

