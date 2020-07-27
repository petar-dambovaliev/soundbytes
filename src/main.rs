mod interpreter;
mod player;

extern crate cpal;
extern crate crossbeam_channel;
extern crate env_logger;
extern crate lazy_static;
extern crate log;
extern crate relative_path;

use interpreter::repl;
use log::error;
use relative_path::RelativePath;
use std::env;
use std::fs::File;
use std::io::stdout;

fn main() {
    env::set_var("RUST_LOG", "soundbytes=info");
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let s = match args.get(1) {
        Some(s) => s,
        None => {
            error!("no input given");
            return;
        }
    };

    let path = RelativePath::new(s);
    let display = path.to_string();

    let file = match File::open(path.to_path(".")) {
        Ok(file) => file,
        Err(why) => {
            error!("couldn't open {}: {}", display, why);
            return;
        }
    };
    repl::start(file, stdout());
}
