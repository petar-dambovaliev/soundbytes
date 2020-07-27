use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::Env;
use std::io::{BufRead, BufReader, Read, Write};
use std::result::Result::Ok;

const PROMPT: &[u8; 3] = b">> ";

pub fn start(in_: impl Read, mut out: impl Write) {
    let buf_reader = BufReader::new(in_);
    let env = Env::new();

    for line in buf_reader.lines() {
        if let Ok(text) = line {
            let _ = out.write(PROMPT);
            let lex = Lexer::new(text.as_str());

            //         evaluated := evaluator.Eval(program, env)
            //         if evaluated != nil {
            //             io.WriteString(out, evaluated.Inspect())
            //             io.WriteString(out, "\n")
            //         }
        }
    }
}

fn print_parser_errors(mut out: impl Write, errors: &[String]) {
    for error in errors {
        if let Err(e) = out.write(error.as_ref()) {
            println!("error writing error {}", e);
        }
    }
}
