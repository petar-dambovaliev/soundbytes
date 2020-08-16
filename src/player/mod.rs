mod clamp;
mod effect;
pub mod instrument;
mod oscillator;
mod play;
mod sound;
mod tempo;

#[derive(Debug)]
pub enum TempoErr {
    InvalidOp {
        left: i32,
        right: i32,
        op: String,
        msg: String,
    },
}

pub struct Tempo {
    pub value: i32,
    pub from_beat: i32,
}

pub struct Song {
    pub start_tempo: i32,
    pub tempo_changes: Vec<Tempo>,
    tempo_cum: i32,
}

impl Song {
    pub fn new() -> Self {
        Self {
            start_tempo: 0,
            tempo_changes: vec![],
            tempo_cum: 0,
        }
    }
    pub fn push_tempo(&mut self, tempo: Tempo) -> Result<(), TempoErr> {
        if tempo.value == 0 {
            return Err(self.err_tempo(tempo.value));
        }
        if self.start_tempo == 0 {
            self.start_tempo = tempo.value;
            self.tempo_cum = tempo.value;
            return Ok(());
        }
        if self.tempo_cum + tempo.value <= 0 {
            return Err(self.err_tempo(tempo.value));
        }
        self.tempo_cum += tempo.value;
        self.tempo_changes.push(tempo);
        Ok(())
    }
    fn err_tempo(&self, tempo: i32) -> TempoErr {
        TempoErr::InvalidOp {
            left: self.tempo_cum,
            right: tempo,
            op: "+".to_string(),
            msg: "allowed values for tempo are above 0.".to_string(),
        }
    }
}
