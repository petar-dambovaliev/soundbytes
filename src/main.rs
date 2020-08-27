mod interpreter;
mod player;

extern crate cpal;
extern crate crossbeam_channel;
extern crate env_logger;
extern crate lazy_static;
extern crate log;
extern crate relative_path;

use interpreter::repl;
use relative_path::RelativePath;
use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Read};

fn main() {
    env::set_var("RUST_LOG", "soundbytes=info");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let mut f = None;
    if let Some(s) = args.get(1) {
        let path = RelativePath::new(s);
        let display = path.to_string();

        let file = match File::open(path.to_path(".")) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        f = Some(file)
    }

    let in_: Box<dyn Read> = match f {
        Some(i) => Box::new(i),
        None => Box::new(stdin()),
    };
    repl::start(in_, stdout());
}
//
// fn bach_bass2() -> VecDeque<Sound> {
//     let sounds = vec![
//         Sound {
//             note: Note::Space,
//             octave: Octave::Three,
//             duration: Duration::EightDotted,
//             effects: None,
//         },
//         Sound {
//             note: Note::Space,
//             octave: Octave::Three,
//             duration: Duration::EightDotted,
//             effects: None,
//         },
//         Sound {
//             note: Note::GSharp,
//             octave: Octave::Two,
//             duration: Duration::Sixteenth,
//             effects: None,
//         },
//         Sound {
//             note: Note::B,
//             octave: Octave::Three,
//             duration: Duration::Sixteenth,
//             effects: None,
//         },
//         Sound {
//             note: Note::D,
//             octave: Octave::Three,
//             duration: Duration::Sixteenth,
//             effects: None,
//         },
//         Sound {
//             note: Note::F,
//             octave: Octave::Three,
//             duration: Duration::Sixteenth,
//             effects: None,
//         },
//         Sound {
//             note: Note::E,
//             octave: Octave::Three,
//             duration: Duration::Sixteenth,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Two,
//             duration: Duration::Sixteenth,
//             effects: None,
//         },
//         Sound {
//             note: Note::E,
//             octave: Octave::Two,
//             duration: Duration::Sixteenth,
//             effects: None,
//         },
//     ];
//     VecDeque::from(sounds)
// }
//
// fn bach_bass() -> VecDeque<Sound> {
//     let sounds = vec![
//         Sound {
//             note: Note::A,
//             octave: Octave::Three,
//             duration: Duration::EightDotted,
//             effects: None,
//         },
//         Sound {
//             note: Note::B,
//             octave: Octave::Three,
//             duration: Duration::EightDotted,
//             effects: None,
//         },
//         Sound {
//             note: Note::GSharp,
//             octave: Octave::Two,
//             duration: Duration::EightDotted,
//             effects: None,
//         },
//         Sound {
//             note: Note::A,
//             octave: Octave::Three,
//             duration: Duration::EightDotted,
//             effects: None,
//         },
//         Sound {
//             note: Note::F,
//             octave: Octave::Two,
//             duration: Duration::EightDotted,
//             effects: None,
//         },
//         Sound {
//             note: Note::FSharp,
//             octave: Octave::Two,
//             duration: Duration::EightDotted,
//             effects: None,
//         },
//         Sound {
//             note: Note::G,
//             octave: Octave::Two,
//             duration: Duration::EightDotted,
//             effects: None,
//         },
//         Sound {
//             note: Note::G,
//             octave: Octave::Two,
//             duration: Duration::EightDotted,
//             effects: None,
//         },
//     ];
//     VecDeque::from(sounds)
// }
//
// fn bach_lead() -> VecDeque<Sound> {
//     let sounds = vec![
//         //A min arpeggio
//         Sound {
//             note: Note::A,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::E,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::A,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::E,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         //B dim arpeggio
//         Sound {
//             note: Note::B,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::D,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::F,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::B,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::D,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::F,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         //G sharp dim arpeggio
//         Sound {
//             note: Note::GSharp,
//             octave: Octave::Four,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::B,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::D,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::GSharp,
//             octave: Octave::Four,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::B,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::D,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         //A min arpeggio
//         Sound {
//             note: Note::A,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::E,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::A,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::E,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         //F maj arpeggio
//         Sound {
//             note: Note::F,
//             octave: Octave::Four,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::A,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::F,
//             octave: Octave::Four,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::A,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         //F# maj arpeggio
//         Sound {
//             note: Note::FSharp,
//             octave: Octave::Four,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::A,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::FSharp,
//             octave: Octave::Four,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::A,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         //transition run
//         Sound {
//             note: Note::F,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::E,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::D,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::B,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::A,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::B,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::C,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::D,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::E,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::F,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//         Sound {
//             note: Note::G,
//             octave: Octave::Five,
//             duration: Duration::ThirtySecond,
//             effects: None,
//         },
//     ];
//     VecDeque::from(sounds)
// }
