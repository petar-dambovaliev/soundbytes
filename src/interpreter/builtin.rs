use crate::interpreter::eval::new_error;
use crate::interpreter::object::{BuiltinObj, Instrument, Object};
use crate::interpreter::object::{Null, Type};
use crate::player::instrument::{InstrumentBox, Options, Synth};
use crate::player::oscillator::AnalogSaw;
use crate::player::play::{PlayErr, Player};
use crate::player::song::Song;
use crate::player::sound::Octave;
use crate::player::sound::{Envelope, Sound as PSound};
use crate::player::tempo::Duration;
use lazy_static::lazy_static;
use log::{error, info, warn};
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

lazy_static! {
    pub static ref TEMPO: Mutex<u32> = Mutex::new(0);
    pub static ref BUILTINS: HashMap<String, BuiltinObj> = {
        let mut hm = HashMap::new();
        hm.insert("tempo".to_string(), BuiltinObj { value: tempo });
        hm.insert("play".to_string(), BuiltinObj { value: play });
        hm.insert("track".to_string(), BuiltinObj { value: track });

        hm
    };
}

fn track(args: Vec<Box<dyn Object + 'static>>, line: usize) -> Box<dyn Object> {
    let ins = match notes_to_ins(args, line) {
        Ok(s) => s,
        Err(e) => return e,
    };
    Box::new(Instrument::new(ins))
}

fn play(args: Vec<Box<dyn Object + 'static>>, line: usize) -> Box<dyn Object> {
    if args.is_empty() {
        return new_error(
            "zero arguments given to play. what am i supposed to play, huh?".to_string(),
            line,
        );
    }

    let first = args.first().unwrap().clone();
    let song = match first.clone().get_type() {
        Type::Instrument(_) => ins_to_song(args, line),
        Type::Sound(_) | Type::Chord(_) => notes_to_song(args, line),
        _ => {
            return new_error(
                format!("invalid argument for play {:?}", first.get_type()),
                line,
            )
        }
    };
    let song = match song {
        Ok(s) => s,
        Err(e) => return e,
    };
    let player = Player::new();
    let err_recv = player.spawn(song).unwrap();

    'outer: loop {
        let res = err_recv.recv();
        if let Ok(e) = res {
            match e {
                PlayErr::StreamErr(stream_err) => warn!("err: {:?}", stream_err),
                PlayErr::BuildStream(build_err) => error!("err: {:?}", build_err),
                PlayErr::EndOfSong => {
                    info!("finished playing all instruments");
                    break 'outer;
                }
            }
        } else if let Err(e) = res {
            warn!("could not receive the end {}", e);
            break 'outer;
        }
    }
    //todo return an error
    Box::new(Null {})
}

fn tempo(mut args: Vec<Box<dyn Object + 'static>>, line: usize) -> Box<dyn Object> {
    if args.len() != 1 {
        return new_error(
            format!("wrong number of arguments. got={}, want=1", args.len()),
            line,
        );
    }

    let arg: Box<dyn Object> = args.pop().unwrap();

    if let Type::Int(tempo) = arg.get_type() {
        if tempo <= 0 {
            return new_error("tempo should be higher than 0".to_string(), line);
        }
        let mut song_tempo = match TEMPO.lock() {
            Ok(s) => s,
            Err(_) => panic!("cannot get song"),
        };
        *song_tempo = tempo as u32;
        return Box::new(Null {});
    }
    //todo return an error
    Box::new(Null {})
}

fn new_opts() -> Options {
    Options {
        osc: Box::new(AnalogSaw::new()),
        env: Envelope::new(),
    }
}

fn notes_to_ins(
    args: Vec<Box<dyn Object + 'static>>,
    line: usize,
) -> Result<InstrumentBox, Box<dyn Object>> {
    if args.is_empty() {
        return Err(new_error(
            "zero arguments given to track. i am expecting notes".to_string(),
            line,
        ));
    }

    let mut sounds = VecDeque::with_capacity(args.len());
    let mut i: usize = 0;
    let mut def_oct = Octave::One;
    let mut def_dur = Duration::Whole;

    match args.first() {
        Some(first) => match first.clone().get_type() {
            Type::Sound(sound) => {
                let s = sound.get_sound();
                def_oct = s.octave.clone();
                def_dur = s.duration;
            }
            Type::Chord(chord) => {
                if let Some(sound) = chord.get_sounds().first() {
                    if sound.modified {
                        return Err(new_error(
                            "expected first note to have an octave and duration".to_string(),
                            line,
                        ));
                    }
                    let s = sound.clone().get_sound();
                    def_oct = s.octave.clone();
                    def_dur = s.duration;
                }
            }
            _ => {
                return Err(new_error(
                    "expected first note to have an octave and duration".to_string(),
                    line,
                ))
            }
        },
        _ => {
            return Err(new_error(
                "expected first note to have an octave and duration".to_string(),
                line,
            ))
        }
    }

    for arg in args {
        let info = arg.inspect();
        match arg.get_type() {
            Type::Sound(sound) => {
                let s = sound.clone().get_sound();
                def_oct = s.octave.clone();
                def_dur = s.duration.clone();
                sounds.push_back(vec![s]);
                i += 1;
            }
            Type::Note(n) => sounds.push_back(vec![PSound::new(
                n.get_note(),
                def_oct.clone(),
                def_dur.clone(),
            )]),
            Type::Chord(chord) => {
                let chord = chord.get_sounds();
                let mut sound = Vec::with_capacity(chord.len());
                for s in chord {
                    sound.push(s.sound);
                }
                sounds.push_back(sound);
            }
            _ => {
                return Err(new_error(
                    format!("expected note, argument {} is {}", i, info),
                    line,
                ));
            }
        }
    }

    Ok(Box::new(Synth::new(new_opts(), sounds)))
}

fn notes_to_song(
    args: Vec<Box<dyn Object + 'static>>,
    line: usize,
) -> Result<Song, Box<dyn Object>> {
    let instr = notes_to_ins(args, line)?;
    let song_tempo = match TEMPO.lock() {
        Ok(s) => s,
        Err(_) => panic!("cannot get song"),
    };
    let mut song = Song::new(*song_tempo);

    song.push_instrument(instr);
    Ok(song)
}

fn ins_to_song(args: Vec<Box<dyn Object + 'static>>, line: usize) -> Result<Song, Box<dyn Object>> {
    let song_tempo = match TEMPO.lock() {
        Ok(s) => s,
        Err(_) => panic!("cannot get song"),
    };

    let mut song = Song::new(*song_tempo);
    let mut i: usize = 0;

    for arg in args {
        let info = arg.inspect();
        if let Type::Instrument(ins) = arg.get_type() {
            song.instruments.push(ins.get_instrument());
            i += 1;
            continue;
        }
        return Err(new_error(
            format!("expected instrument, argument {} is {}", i, info),
            line,
        ));
    }

    Ok(song)
}
