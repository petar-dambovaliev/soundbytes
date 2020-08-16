use crate::interpreter::eval::new_error;
use crate::interpreter::object::{BuiltinObj, Object};
use crate::interpreter::object::{Null, Type};
use crate::player::Song;
use crate::player::Tempo;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref SONG: Mutex<Song> = Mutex::new(Song::new());
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
                        let mut song = match SONG.lock() {
                            Ok(s) => s,
                            Err(_) => panic!("cannot get song"),
                        };
                        if let Err(e) = song.push_tempo(Tempo {
                            value: tempo,
                            from_beat: 0,
                        }) {
                            panic!("{:?}", e);
                        }
                    }
                    Box::new(Null {})
                },
            },
        );
        hm
    };
}
