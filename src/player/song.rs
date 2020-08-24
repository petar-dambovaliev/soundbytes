use crate::player::instrument::{InstrumentBox, Instruments};
use crate::player::tempo::{Tempo, TempoErr};

pub struct Song {
    pub start_tempo: u32,
    pub tempo_changes: Vec<Tempo>,
    tempo_cum: u32,
    pub instruments: Instruments,
}

impl Song {
    pub fn new(tempo: u32) -> Self {
        Self {
            start_tempo: tempo,
            tempo_changes: vec![],
            tempo_cum: 0,
            instruments: vec![],
        }
    }

    pub fn push_instrument(&mut self, i: InstrumentBox) {
        self.instruments.push(i)
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
    fn err_tempo(&self, tempo: u32) -> TempoErr {
        TempoErr::InvalidOp {
            left: self.tempo_cum,
            right: tempo,
            op: "+".to_string(),
            msg: "allowed values for tempo are above 0.".to_string(),
        }
    }
}
