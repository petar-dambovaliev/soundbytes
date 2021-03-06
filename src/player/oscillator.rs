use crate::player::clamp::Clamp;
use crate::player::tempo::SampleClock;
use std::f32::consts::PI;
use std::fmt::Debug;

pub type OscillatorBox = Box<dyn Oscillator>;

pub trait Oscillator: Debug + Send + CloneOsc {
    fn oscillator(&mut self, hz: f32, sample_rate: f32, sample_clock: &SampleClock) -> f32;
}

pub trait CloneOsc {
    fn clone_osc(&self) -> OscillatorBox;
}

impl<T> CloneOsc for T
where
    T: Oscillator + Clone + 'static,
{
    fn clone_osc(&self) -> OscillatorBox {
        Box::new(self.clone())
    }
}

impl Clone for OscillatorBox {
    fn clone(&self) -> Self {
        self.clone_osc()
    }
}

#[derive(Debug, Clone)]
pub struct AnalogSaw {}

#[allow(dead_code)]
impl AnalogSaw {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Oscillator for AnalogSaw {
    fn oscillator(&mut self, hz: f32, sample_rate: f32, sample_clock: &SampleClock) -> f32 {
        let mut output = 0.0_f32;

        for i in 1..40 {
            let n = i as f32;
            output += (n * to_angular_vel_rate(hz, sample_rate, sample_clock)).sin() / n
        }

        (output * (2.0 / PI)).my_clamp(-1.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct TriangleWave {}

#[allow(dead_code)]
impl TriangleWave {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

fn to_angular_vel_rate(hz: f32, sample_rate: f32, sample_clock: &SampleClock) -> f32 {
    (2.0 * PI * hz) * sample_clock.get_clock() / sample_rate
}

impl Oscillator for TriangleWave {
    fn oscillator(&mut self, hz: f32, sample_rate: f32, sample_clock: &SampleClock) -> f32 {
        let res = to_angular_vel_rate(hz, sample_rate, sample_clock)
            .sin()
            .asin()
            * (2.0 / PI);
        res.my_clamp(-1.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct SinWave {}

#[allow(dead_code)]
impl SinWave {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Oscillator for SinWave {
    fn oscillator(&mut self, hz: f32, sample_rate: f32, sample_clock: &SampleClock) -> f32 {
        to_angular_vel_rate(hz, sample_rate, sample_clock)
            .sin()
            .my_clamp(-1.0, 1.0)
    }
}
