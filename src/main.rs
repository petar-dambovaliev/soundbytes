mod interpreter;
mod player;

extern crate cpal;
extern crate crossbeam_channel;
extern crate env_logger;
extern crate lazy_static;
extern crate log;

use crate::instrument::{InstrumentBox, Instruments, Options, Synth};
use crate::oscillator::AnalogSaw;
use crate::play::PlayErr;
use crate::sound::{Envelope, Note, Octave, Sound};
use crate::tempo::Duration;
use cpal::DefaultStreamConfigError;
use log::{error, info, warn};
use play::Player;
use std::collections::VecDeque;

fn main() -> Result<(), DefaultStreamConfigError> {
    std::env::set_var("RUST_LOG", "soundbytes=info");
    env_logger::init();

    let synth_lead: InstrumentBox = Box::new(Synth::new(new_opts(), bach_lead()));
    let synth_bass: InstrumentBox = Box::new(Synth::new(new_opts(), bach_bass()));
    let synth_bass2: InstrumentBox = Box::new(Synth::new(new_opts(), bach_bass2()));
    let instruments: Instruments = vec![synth_bass2, synth_lead, synth_bass];

    let player = Player::new();
    let err_recv = player.spawn(instruments, 30.0)?;

    loop {
        let res = err_recv.recv();
        if let Ok(e) = res {
            match e {
                PlayErr::StreamErr(stream_err) => warn!("err: {:?}", stream_err),
                PlayErr::BuildStream(build_err) => error!("err: {:?}", build_err),
                PlayErr::EndOfSong => {
                    info!("finished playing all instruments");
                    return Ok(());
                }
            }
        } else if let Err(e) = res {
            warn!("could not receive the end {}", e);
            return Ok(());
        }
    }
}

fn new_opts() -> Options {
    let opts = Options {
        osc: Box::new(AnalogSaw::new()),
        env: Envelope::new(),
    };
    opts
}

fn bach_bass2() -> VecDeque<Sound> {
    let sounds = vec![
        Sound {
            note: Note::Space,
            octave: Octave::Three,
            duration: Duration::EightDotted,
            effects: None,
        },
        Sound {
            note: Note::Space,
            octave: Octave::Three,
            duration: Duration::EightDotted,
            effects: None,
        },
        Sound {
            note: Note::GSharp,
            octave: Octave::Two,
            duration: Duration::Sixteenth,
            effects: None,
        },
        Sound {
            note: Note::B,
            octave: Octave::Three,
            duration: Duration::Sixteenth,
            effects: None,
        },
        Sound {
            note: Note::D,
            octave: Octave::Three,
            duration: Duration::Sixteenth,
            effects: None,
        },
        Sound {
            note: Note::F,
            octave: Octave::Three,
            duration: Duration::Sixteenth,
            effects: None,
        },
        Sound {
            note: Note::E,
            octave: Octave::Three,
            duration: Duration::Sixteenth,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Two,
            duration: Duration::Sixteenth,
            effects: None,
        },
        Sound {
            note: Note::E,
            octave: Octave::Two,
            duration: Duration::Sixteenth,
            effects: None,
        },
    ];
    VecDeque::from(sounds)
}

fn bach_bass() -> VecDeque<Sound> {
    let sounds = vec![
        Sound {
            note: Note::A,
            octave: Octave::Three,
            duration: Duration::EightDotted,
            effects: None,
        },
        Sound {
            note: Note::B,
            octave: Octave::Three,
            duration: Duration::EightDotted,
            effects: None,
        },
        Sound {
            note: Note::GSharp,
            octave: Octave::Two,
            duration: Duration::EightDotted,
            effects: None,
        },
        Sound {
            note: Note::A,
            octave: Octave::Three,
            duration: Duration::EightDotted,
            effects: None,
        },
        Sound {
            note: Note::F,
            octave: Octave::Two,
            duration: Duration::EightDotted,
            effects: None,
        },
        Sound {
            note: Note::FSharp,
            octave: Octave::Two,
            duration: Duration::EightDotted,
            effects: None,
        },
        Sound {
            note: Note::G,
            octave: Octave::Two,
            duration: Duration::EightDotted,
            effects: None,
        },
        Sound {
            note: Note::G,
            octave: Octave::Two,
            duration: Duration::EightDotted,
            effects: None,
        },
    ];
    VecDeque::from(sounds)
}

fn bach_lead() -> VecDeque<Sound> {
    let sounds = vec![
        //A min arpeggio
        Sound {
            note: Note::A,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::E,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::A,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::E,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        //B dim arpeggio
        Sound {
            note: Note::B,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::D,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::F,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::B,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::D,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::F,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        //G sharp dim arpeggio
        Sound {
            note: Note::GSharp,
            octave: Octave::Four,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::B,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::D,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::GSharp,
            octave: Octave::Four,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::B,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::D,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        //A min arpeggio
        Sound {
            note: Note::A,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::E,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::A,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::E,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        //F maj arpeggio
        Sound {
            note: Note::F,
            octave: Octave::Four,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::A,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::F,
            octave: Octave::Four,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::A,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        //F# maj arpeggio
        Sound {
            note: Note::FSharp,
            octave: Octave::Four,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::A,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::FSharp,
            octave: Octave::Four,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::A,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        //transition run
        Sound {
            note: Note::F,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::E,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::D,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::B,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::A,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::B,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::C,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::D,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::E,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::F,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
        Sound {
            note: Note::G,
            octave: Octave::Five,
            duration: Duration::ThirtySecond,
            effects: None,
        },
    ];
    VecDeque::from(sounds)
}
