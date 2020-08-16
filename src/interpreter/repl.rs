use crate::interpreter::eval::eval;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::Env;
use crate::interpreter::parser::Parser;
use std::io::{BufRead, BufReader, Read, Write};
use std::result::Result::Ok;

const PROMPT: &[u8; 3] = b">> ";

#[allow(dead_code)]
pub fn start(in_: impl Read, mut out: impl Write + std::fmt::Write) {
    let buf_reader = BufReader::new(in_);
    let env = Env::new();

    for line in buf_reader.lines() {
        if let Ok(text) = line {
            let _ = out.write(PROMPT);

            let lex = Lexer::new(text.as_str());
            let mut p = Parser::new(lex);

            let program = Box::new(p.parse_program());
            for expr in program.exprs {
                let evaluated = eval(expr.to_node(), &env);
                let _ = out.write_str(&evaluated.inspect());
                let _ = out.write_char('\n');
            }
        }
    }
}

#[allow(dead_code)]
fn print_parser_errors(mut out: impl Write, errors: &[String]) {
    for error in errors {
        if let Err(e) = out.write(error.as_ref()) {
            println!("error writing error {}", e);
        }
    }
}
