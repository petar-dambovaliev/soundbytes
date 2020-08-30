use super::sound::Sound;
use crate::player::effect::EffectBox;
use crate::player::oscillator::OscillatorBox;
use crate::player::sound::{Envelope, Frequency};
use crate::player::tempo::{calc_duration, Rates, SampleClock};
use std::collections::VecDeque;
use std::fmt::Debug;

pub trait Instrument: Debug + Send + CloneIns {
    fn next_freq(&mut self, sample_rate: f32, beat_per_min: f32) -> f32;
    fn is_playing(&self) -> bool;
    fn is_finished(&self) -> bool;
}

pub trait CloneIns {
    fn clone_ins(&self) -> InstrumentBox;
}

impl<T> CloneIns for T
where
    T: Instrument + Clone + 'static,
{
    fn clone_ins(&self) -> InstrumentBox {
        Box::new(self.clone())
    }
}

impl Clone for InstrumentBox {
    fn clone(&self) -> Self {
        self.clone_ins()
    }
}

pub type InstrumentBox = Box<dyn Instrument>;
pub type Instruments = Vec<InstrumentBox>;

#[derive(Debug, Clone)]
pub struct Options {
    pub(crate) osc: OscillatorBox,
    pub(crate) env: Envelope,
}

#[derive(Debug, Clone)]
struct Inner {
    score: VecDeque<Sound>,
    effects: Option<Vec<EffectBox>>,
    freq: f32,
    end: f32,
    finished: bool,
    sample_clock: SampleClock,
}

#[derive(Debug, Clone)]
pub struct Synth {
    inner: Inner,
    opts: Options,
}

#[allow(dead_code)]
impl Synth {
    pub fn new(opts: Options, score: VecDeque<Sound>) -> Self {
        Self {
            inner: Inner {
                score,
                effects: None,
                freq: 0.0,
                end: 0.0,
                finished: false,
                sample_clock: SampleClock::new(),
            },
            opts,
        }
    }

    fn next_frequency(&mut self, sample_rate: f32, beat_per_min: f32) -> Option<Frequency> {
        let sound = self.inner.score.pop_front()?;

        let beat_frame_dur = calc_duration(Rates {
            sample_rate,
            beat_per_min,
            duration: &sound.duration,
        });

        let frequency = sound.note.frequency(sound.octave);
        self.inner.sample_clock.calc_end(beat_frame_dur);
        let effects = sound.effects;

        Some(Frequency { frequency, effects })
    }
}

impl Instrument for Synth {
    fn next_freq(&mut self, sample_rate: f32, beat_per_min: f32) -> f32 {
        if !self.is_playing() {
            if let Some(f) = self.next_frequency(sample_rate, beat_per_min) {
                self.inner.freq = f.frequency;
                self.inner.effects = f.effects;
            } else {
                self.inner.freq = 0.0;
                self.inner.finished = true;
            }
        }

        let sc = &mut self.inner.sample_clock;
        let freq = apply_effects(self.inner.freq, &self.inner.effects, sc);
        sc.update_clock();
        apply_options(freq, &mut self.opts, sample_rate, sc)
    }

    fn is_playing(&self) -> bool {
        self.inner.sample_clock.get_clock() as u32 != self.inner.end as u32
    }

    fn is_finished(&self) -> bool {
        self.inner.finished
    }
}

fn apply_effects(freq: f32, effects: &Option<Vec<EffectBox>>, sample_clock: &SampleClock) -> f32 {
    let effects = match effects {
        Some(e) => e,
        None => return freq,
    };
    let mut eff_f = 0.0;
    for effect in effects.iter() {
        eff_f += effect.get_frequency(&sample_clock)
    }
    freq + eff_f
}

fn apply_options(
    freq: f32,
    opts: &mut Options,
    sample_rate: f32,
    sample_clock: &SampleClock,
) -> f32 {
    let env_f = opts.env.get_amplitude(&sample_clock);
    let osc_f = opts.osc.oscillator(freq, sample_rate, sample_clock);

    osc_f * env_f
}
