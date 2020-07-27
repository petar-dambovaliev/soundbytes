use crate::player::effect::EffectBox;
use crate::player::tempo::{Duration, SampleClock};

const A: f32 = 440.0;

#[derive(Debug, Clone)]
pub struct Envelope {
    attack_time: f32,
    decay_time: f32,
    sustain_amplitude: f32,
    release_time: f32,
    start_amplitude: f32,
    trigger_off_time: f32,
    trigger_on_time: f32,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            attack_time: 0.10,
            decay_time: 0.01,
            sustain_amplitude: 0.8,
            release_time: 0.20,
            start_amplitude: 1.0,
            trigger_off_time: 0.0,
            trigger_on_time: 0.0,
        }
    }

    pub fn get_amplitude(&self, sample_clock: &SampleClock) -> f32 {
        let mut amplitude = 0.0;
        let life_time = sample_clock.get_clock() - self.trigger_on_time;

        if life_time <= self.attack_time {
            // In attack Phase - approach max amplitude
            amplitude = (life_time / self.attack_time) * self.start_amplitude;
        }

        if life_time > self.attack_time && life_time <= (self.attack_time + self.decay_time) {
            // In decay phase - reduce to sustained amplitude
            amplitude = ((life_time - self.attack_time) / self.decay_time)
                * (self.sustain_amplitude - self.start_amplitude)
                + self.start_amplitude;
        }

        if life_time > (self.attack_time + self.decay_time) {
            // In sustain phase - dont change until note released
            amplitude = self.sustain_amplitude;
        }
        if amplitude <= 0.0001 {
            return 0.0;
        }

        amplitude
    }
}

#[derive(Debug)]
pub struct Frequency {
    pub frequency: f32,
    pub effects: Option<Vec<EffectBox>>,
}

#[derive(Debug)]
pub struct Sound {
    pub(crate) note: Note,
    pub(crate) octave: Octave,
    pub(crate) duration: Duration,
    pub(crate) effects: Option<Vec<EffectBox>>,
}

impl Note {
    pub fn frequency(self, octave: Octave) -> f32 {
        if let Note::Space = self {
            return 0.0;
        }
        let float = (self as isize - 1 + octave as isize) as f32 / 12.0;
        A * float.exp2()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Note {
    Space,
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Octave {
    One = -36,
    Two = -24,
    Three = -12,
    Four = 0,
    Five = 12,
    Six = 24,
    Seven = 36,
    Eight = 48,
}
