const SEC_PER_MIN: f32 = 60.0;

#[derive(Debug)]
pub enum TempoErr {
    InvalidOp {
        left: u32,
        right: u32,
        op: String,
        msg: String,
    },
}

pub struct Tempo {
    pub value: u32,
    pub from_beat: u32,
}

pub struct Rates<'a> {
    pub(crate) sample_rate: f32,
    pub(crate) beat_per_min: f32,
    pub(crate) duration: &'a Duration,
}

pub fn calc_duration(rates: Rates) -> f32 {
    let rate_per_beat: f32 = rates.sample_rate / (rates.beat_per_min / SEC_PER_MIN);
    rate_per_beat * rates.duration.to_beats()
}

#[derive(Debug, Clone)]
pub enum Duration {
    Whole,
    HalfDotted,
    Half,
    QuarterDotted,
    Quarter,
    EightDotted,
    Eight,
    SixteenthDotted,
    Sixteenth,
    ThirtySecondDotted,
    ThirtySecond,
}

impl Duration {
    pub fn to_beats(&self) -> f32 {
        match self {
            Self::Whole => 4.0,
            Self::HalfDotted => 3.0,
            Self::Half => 2.0,
            Self::QuarterDotted => 1.5,
            Self::Quarter => 1.0,
            Self::EightDotted => 0.75,
            Self::Eight => 0.5,
            Self::SixteenthDotted => 0.375,
            Self::Sixteenth => 0.25,
            Self::ThirtySecondDotted => 0.1875,
            Self::ThirtySecond => 0.125,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SampleClock {
    sample_clock: f32,
    end: f32,
}

#[allow(dead_code)]
impl SampleClock {
    pub(crate) fn new() -> Self {
        Self {
            sample_clock: 0.0,
            end: 0.0,
        }
    }
    pub(crate) fn update_clock(&mut self) {
        if self.sample_clock as u32 >= self.end as u32 {
            self.sample_clock = 0.0;
            return;
        }
        self.sample_clock += 1.0;
    }
    pub(crate) fn calc_end(&mut self, dur: f32) {
        self.end = self.get_clock() + dur;
    }
    pub(crate) fn get_clock(&self) -> f32 {
        self.sample_clock
    }
}
