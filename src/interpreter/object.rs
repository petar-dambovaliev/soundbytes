use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};
use std::option::Option::Some;

pub enum Type {
    Int(i32),
    Float(f32),
    String(String),
    Builtin(DefaultBuiltinFunc),
    Error(String),
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Int(i) => f.write_str(&format!("Int({})", i)),
            Self::Float(i) => f.write_str(&format!("Float({})", i)),
            Self::String(i) => f.write_str(&format!("String({})", i)),
            Self::Builtin(i) => f.write_str("Builtin()"),
            Self::Error(i) => f.write_str(&format!("Error({})", i)),
        }
    }
}

pub trait Object {
    fn get_type(self) -> Type;
    fn inspect(&self) -> String;
    fn is_error(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct IntObj {
    pub(crate) value: i32,
}

impl Object for IntObj {
    fn get_type(self) -> Type {
        Type::Int(self.value)
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Clone, Debug)]
pub struct FloatObj {
    pub(crate) value: f32,
}

impl Object for FloatObj {
    fn get_type(self) -> Type {
        Type::Float(self.value)
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug)]
pub struct StringObj {
    pub(crate) value: String,
}

impl Object for StringObj {
    fn get_type(self) -> Type {
        Type::String(self.value)
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

pub trait BuiltinFn: Fn(Vec<Box<dyn Object>>) -> Box<dyn Object> + Sync {}
impl BuiltinFn for fn(Vec<Box<dyn Object>>) -> Box<dyn Object> {}
type DefaultBuiltinFunc = fn(Vec<Box<dyn Object>>) -> Box<dyn Object>;

pub struct BuiltinObj {
    pub(crate) value: DefaultBuiltinFunc,
}

impl Object for BuiltinObj {
    fn get_type(self) -> Type {
        Type::Builtin(self.value)
    }

    fn inspect(&self) -> String {
        "builtin function".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Error {
    pub(crate) msg: String,
}

impl Object for Error {
    fn get_type(self) -> Type {
        Type::Error(self.msg)
    }

    fn inspect(&self) -> String {
        format!("ERROR: {}", self.msg)
    }
    fn is_error(&self) -> bool {
        true
    }
}

pub struct Env<'a> {
    store: HashMap<String, &'a dyn Object>,
    outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn new_enclosed(outer: &'a Env<'a>) -> Self {
        Self {
            store: Default::default(),
            outer: Some(outer),
        }
    }
    pub fn new() -> Self {
        Self {
            store: Default::default(),
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<&dyn Object> {
        if let Some(&s) = self.store.get(name) {
            return Some(s);
        }
        if let Some(outer) = self.outer {
            if let Some(&outer_res) = outer.store.get(name) {
                return Some(outer_res);
            }
        }
        None
    }

    pub fn set(&mut self, name: String, obj: &'a dyn Object) {
        self.store.insert(name, obj);
    }
}
