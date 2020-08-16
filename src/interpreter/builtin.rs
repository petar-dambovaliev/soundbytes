use crate::interpreter::eval::new_error;
use crate::interpreter::object::Type;
use crate::interpreter::object::{BuiltinObj, Object};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref BUILTINS: HashMap<String, BuiltinObj> = {
        let mut hm = HashMap::new();
        hm.insert(
            "tempo".to_string(),
            BuiltinObj {
                value: |args: Vec<Box<dyn Object>>| -> Box<dyn Object> {
                    if args.len() != 1 {
                        return new_error(format!(
                            "wrong number of arguments. got={}, want=1",
                            args.len()
                        ));
                    }

                    let arg = args.first().unwrap();

                    // if let Type::Int(i) = arg.get_type() {}

                    unimplemented!()
                },
            },
        );
        hm
    };
}
