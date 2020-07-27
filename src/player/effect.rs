use crate::player::tempo::SampleClock;
use std::fmt::Debug;

pub type EffectBox = Box<dyn Effect>;

pub trait Effect: Debug + Send + CloneEffect + 'static {
    fn get_frequency(&self, sample_clock: &SampleClock) -> f32;
}

pub trait CloneEffect {
    fn clone_effect(&self) -> EffectBox;
}

impl<T> CloneEffect for T
where
    T: Effect + Clone + 'static,
{
    fn clone_effect(&self) -> EffectBox {
        Box::new(self.clone())
    }
}

impl Clone for EffectBox {
    fn clone(&self) -> Self {
        self.clone_effect()
    }
}

#[derive(Debug, Clone)]
pub struct Vibrato {
    depth: f32,
    phase: f32,
    speed: f32,
}

#[allow(dead_code)]
impl Vibrato {
    pub fn new(depth: f32, phase: f32) -> Self {
        Self {
            depth,
            phase,
            speed: 0.00000001,
        }
    }
}

impl Effect for Vibrato {
    fn get_frequency(&self, sample_clock: &SampleClock) -> f32 {
        (sample_clock.get_clock() * 0.001).sin()
            * self.depth
            * sample_clock.get_clock()
            * (self.speed + 0.0000005)
    }
}
