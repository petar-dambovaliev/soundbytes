use super::sound::Sound;
use crate::player::effect::EffectBox;
use crate::player::oscillator::OscillatorBox;
use crate::player::sound::{Envelope, Frequency};
use crate::player::tempo::{calc_duration, Rates, SampleClock};
use std::collections::VecDeque;
use std::fmt::Debug;

pub trait Instrument: Debug + Send + CloneIns {
    fn next_freq(&mut self, sample_rate: f32, beat_per_min: f32) -> f32;
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
struct InnerSound {
    sample_clock: SampleClock,
    end: f32,
    freq: f32,
    effects: Option<Vec<EffectBox>>,
    sample_rate: f32,
}

impl InnerSound {
    fn new(sound: Sound, sample_rate: f32, beat_per_min: f32) -> Self {
        let beat_frame_dur = calc_duration(Rates {
            sample_rate,
            beat_per_min,
            duration: &sound.duration,
        });

        let freq = sound.note.frequency(sound.octave);
        let mut sample_clock = SampleClock::new();
        sample_clock.calc_end(beat_frame_dur);
        let effects = sound.effects;

        Self {
            sample_clock,
            end: 0.0,
            freq,
            effects,
            sample_rate,
        }
    }
    fn next_freq(&mut self) -> f32 {
        if self.has_ended() {
            return 0.0;
        }

        let sc = &mut self.sample_clock;
        let freq = apply_effects(self.freq, &self.effects, sc);
        sc.update_clock();
        freq
    }

    fn has_ended(&self) -> bool {
        self.sample_clock.get_clock() as u32 != self.end as u32
    }
}

#[derive(Debug, Clone)]
pub struct Synth {
    score: VecDeque<Vec<Sound>>,
    cur: Vec<InnerSound>,
    first_finished: bool,
    finished: bool,
    opts: Options,
}

#[allow(dead_code)]
impl Synth {
    pub fn new(opts: Options, score: VecDeque<Vec<Sound>>) -> Self {
        Self {
            score,
            cur: vec![],
            first_finished: true,
            finished: false,
            opts,
        }
    }

    fn next_frequency(&mut self, sample_rate: f32, beat_per_min: f32) -> f32 {
        let mut first_index: usize = 0;
        if self.first_finished {
            if let Some(sounds) = self.score.pop_front() {
                first_index = self.cur.len();

                for sound in sounds {
                    self.cur
                        .push(InnerSound::new(sound, sample_rate, beat_per_min));
                }
                self.first_finished = false;
            }
        }

        if let Some(cur) = self.cur.get(first_index) {
            self.first_finished = cur.has_ended();
        }

        self.cur.retain(|s| !s.has_ended());

        let mut f = 0.0_f32;
        for cur_sound in self.cur.iter_mut() {
            f += apply_options(
                cur_sound.next_freq(),
                &mut self.opts,
                sample_rate,
                &cur_sound.sample_clock,
            );
        }

        f
    }
}

impl Instrument for Synth {
    fn next_freq(&mut self, sample_rate: f32, beat_per_min: f32) -> f32 {
        let freq = self.next_frequency(sample_rate, beat_per_min);

        if freq as u32 == 0 {
            self.finished = true;
            return 0.0;
        }
        freq
    }

    fn is_finished(&self) -> bool {
        self.finished
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
