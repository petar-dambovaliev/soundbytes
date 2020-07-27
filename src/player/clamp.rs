pub trait Clamp {
    fn my_clamp(self, min: f32, max: f32) -> f32;
}

impl Clamp for f32 {
    #[inline]
    fn my_clamp(self, min: f32, max: f32) -> f32 {
        assert!(min <= max);
        let mut x = self;
        if x < min {
            x = min;
        }
        if x > max {
            x = max;
        }
        x
    }
}
