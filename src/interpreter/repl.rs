use crate::interpreter::eval::eval;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::{Duration, Env, Note, Octave};
use crate::interpreter::parser::Parser;
use crate::player::sound::{Note as PNote, Octave as POctave};
use crate::player::tempo::Duration as PDUration;
use std::io::{BufReader, Read, Write};

pub fn start(in_: impl Read, mut out: impl Write) {
    let mut buf_reader = BufReader::new(in_);
    let mut env = Env::new();
    inject_predeclared(&mut env);

    let mut input = String::new();
    let _ = buf_reader.read_to_string(&mut input);

    let _ = out.write(b"playing your music\n");

    let lex = Lexer::new(&input);
    let mut p = Parser::new(lex);

    let program = Box::new(p.parse_program());
    for expr in program.exprs {
        let evaluated = eval(expr.to_node(), &mut env);
        let _ = out.write(evaluated.inspect().as_bytes());
        let _ = out.write(b"\n");
    }
}

fn inject_predeclared(env: &mut Env) {
    let mut inject_note = |n: PNote, k: &str| {
        env.set(k.to_string(), Box::new(Note::new(n)));
    };

    inject_note(PNote::Space, "x");
    inject_note(PNote::A, "a");
    inject_note(PNote::ASharp, "a#");
    inject_note(PNote::B, "b");
    inject_note(PNote::C, "c");
    inject_note(PNote::CSharp, "c#");
    inject_note(PNote::D, "d");
    inject_note(PNote::DSharp, "d#");
    inject_note(PNote::E, "e");
    inject_note(PNote::F, "f");
    inject_note(PNote::FSharp, "f#");
    inject_note(PNote::G, "g");
    inject_note(PNote::GSharp, "g#");

    let mut inject_octave = |n: POctave, k: &str| {
        env.set(k.to_string(), Box::new(Octave::new(n)));
    };

    inject_octave(POctave::One, "o1");
    inject_octave(POctave::Two, "o2");
    inject_octave(POctave::Three, "o3");
    inject_octave(POctave::Four, "o4");
    inject_octave(POctave::Five, "o5");
    inject_octave(POctave::Six, "o6");
    inject_octave(POctave::Seven, "o7");
    inject_octave(POctave::Eight, "o8");

    let mut inject_dur = |n: PDUration, k: &str| {
        env.set(k.to_string(), Box::new(Duration::new(n)));
    };

    inject_dur(PDUration::Whole, "d1");
    inject_dur(PDUration::HalfDotted, "d2*");
    inject_dur(PDUration::Half, "d2");
    inject_dur(PDUration::QuarterDotted, "d4*");
    inject_dur(PDUration::Quarter, "d4");
    inject_dur(PDUration::EightDotted, "d8*");
    inject_dur(PDUration::Eight, "d8");
    inject_dur(PDUration::SixteenthDotted, "d16*");
    inject_dur(PDUration::Sixteenth, "d16");
    inject_dur(PDUration::ThirtySecondDotted, "d32*");
    inject_dur(PDUration::ThirtySecond, "d32");
}

#[allow(dead_code)]
fn print_parser_errors(mut out: impl Write, errors: &[String]) {
    for error in errors {
        if let Err(e) = out.write(error.as_ref()) {
            println!("error writing error {}", e);
        }
    }
}
