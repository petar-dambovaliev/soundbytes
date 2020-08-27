use crate::interpreter::eval::new_error;
use crate::interpreter::object::{BuiltinObj, Object};
use crate::interpreter::object::{Null, Type};
use crate::player::instrument::{InstrumentBox, Options, Synth};
use crate::player::oscillator::AnalogSaw;
use crate::player::play::{PlayErr, Player};
use crate::player::song::Song;
use crate::player::sound::Envelope;
use lazy_static::lazy_static;
use log::{error, info, warn};
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

fn new_opts() -> Options {
    Options {
        osc: Box::new(AnalogSaw::new()),
        env: Envelope::new(),
    }
}

lazy_static! {
    pub static ref TEMPO: Mutex<u32> = Mutex::new(0);
    pub static ref BUILTINS: HashMap<String, BuiltinObj> = {
        let mut hm = HashMap::new();
        hm.insert(
            "tempo".to_string(),
            BuiltinObj {
                value: |mut args: Vec<Box<dyn Object + 'static>>| -> Box<dyn Object> {
                    if args.len() != 1 {
                        return new_error(format!(
                            "wrong number of arguments. got={}, want=1",
                            args.len()
                        ));
                    }

                    let arg: Box<dyn Object> = args.pop().unwrap();

                    if let Type::Int(tempo) = arg.get_type() {
                        if tempo <= 0 {
                            return new_error("tempo should be higher than 0".to_string());
                        }
                        let mut song_tempo = match TEMPO.lock() {
                            Ok(s) => s,
                            Err(_) => panic!("cannot get song"),
                        };
                        *song_tempo  = tempo as u32;
                        return Box::new(Null {});
                    }
                    //todo return an error
                    Box::new(Null {})
                },
            },
        );

        hm.insert(
            "play".to_string(),
            BuiltinObj {
                value: |args: Vec<Box<dyn Object + 'static>>| -> Box<dyn Object> {
                    if args.is_empty() {
                        return new_error(
                            "zero arguments given to play. what am i supposed to play, huh?"
                                .to_string(),
                        );
                    }

                    let mut sounds = VecDeque::with_capacity(args.len());
                    let mut i: usize = 0;
                    for arg in args {
                        let info = arg.inspect();
                        if let Type::Sound(sound) = arg.get_type() {
                           sounds.push_back(sound.get_sound());
                           i += 1;
                           continue;
                        }
                        return new_error(format!("expected note, argument {} is {}", i, info))
                    }
                    let song_tempo = match TEMPO.lock() {
                            Ok(s) => s,
                            Err(_) => panic!("cannot get song"),
                        };
                    let mut song = Song::new(*song_tempo);

                    let instr: InstrumentBox = Box::new(Synth::new(new_opts(), sounds));
                    song.push_instrument(instr);
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
                },
            },
        );

        // hm.insert(
        //     "time".to_string(),
        //     BuiltinObj {
        //         value: |mut args: Vec<Box<dyn Object + 'static>>| -> Box<dyn Object> {
        //             if args.len() != 1 {
        //                 return new_error(
        //                     "zero arguments given to time. i am expecting a time signature."
        //                         .to_string(),
        //                 );
        //             }
        //
        //             let arg: Box<dyn Object> = args.pop().unwrap();
        //
        //             if let Type::TimeSignature(time_sig) = arg.get_type() {
        //                 let mut song = match SONG.lock() {
        //                     Ok(s) => s,
        //                     Err(_) => panic!("cannot get song"),
        //                 };
        //                 song.
        //             }
        //             Box::new(Null {})
        //         },
        //     },
        // );

        hm
    };
}
