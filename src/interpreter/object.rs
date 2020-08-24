use crate::player::sound::{Note as PNote, Octave as POctave, Sound as PSound};
use crate::player::tempo::Duration as PDuration;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};

pub enum Type {
    Int(i32),
    Float(f32),
    String(String),
    Builtin(DefaultBuiltinFunc),
    TimeSignature(TimeSignature),
    Error(String),
    Sound(Sound),
    Note(Note),
    Octave(Octave),
    Duration(Duration),
    Null,
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Int(i) => f.write_str(&format!("Int({})", i)),
            Self::Float(i) => f.write_str(&format!("Float({})", i)),
            Self::String(i) => f.write_str(&format!("String({})", i)),
            Self::Builtin(_) => f.write_str("Builtin()"),
            Self::TimeSignature(ts) => f.write_str(&format!("TimeSignature({}/{})", ts.n, ts.dur)),
            Self::Error(i) => f.write_str(&format!("Error({})", i)),
            Self::Sound(n) => f.write_str(&n.inspect()),
            Self::Note(n) => f.write_str(&n.inspect()),
            Self::Octave(n) => f.write_str(&n.inspect()),
            Self::Duration(n) => f.write_str(&n.inspect()),
            Self::Null => f.write_str("Null"),
        }
    }
}

pub trait Object: CloneObj + Debug {
    fn get_type(self: Box<Self>) -> Type;
    fn inspect(&self) -> String;
    fn is_error(&self) -> bool {
        false
    }
}

pub trait CloneObj {
    fn clone_obj(&self) -> Box<dyn Object>;
}

impl<T> CloneObj for T
where
    T: Object + Clone + 'static,
{
    fn clone_obj(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.clone_obj()
    }
}

#[derive(Clone, Debug)]
pub struct Duration {
    dur: PDuration,
}

impl Duration {
    pub fn new(dur: PDuration) -> Self {
        Self { dur }
    }
    pub fn get_dur(&self) -> PDuration {
        self.dur.clone()
    }
}

impl Object for Duration {
    fn get_type(self: Box<Self>) -> Type {
        Type::Duration(*self)
    }

    fn inspect(&self) -> String {
        format!("Note: {:?}", self.dur)
    }
}

#[derive(Clone, Debug)]
pub struct Octave {
    octave: POctave,
}

impl Octave {
    pub fn new(octave: POctave) -> Self {
        Self { octave }
    }
    pub fn get_oct(&self) -> POctave {
        self.octave.clone()
    }
}

impl Object for Octave {
    fn get_type(self: Box<Self>) -> Type {
        Type::Octave(*self)
    }

    fn inspect(&self) -> String {
        format!("Note: {:?}", self.octave)
    }
}

#[derive(Clone, Debug)]
pub struct Note {
    note: PNote,
}

impl Note {
    pub fn new(note: PNote) -> Self {
        Self { note }
    }
    pub fn get_note(&self) -> PNote {
        self.note.clone()
    }
}

impl Object for Note {
    fn get_type(self: Box<Self>) -> Type {
        Type::Note(*self)
    }

    fn inspect(&self) -> String {
        format!("Note: {:?}", self.note)
    }
}

#[derive(Clone, Debug)]
pub struct Sound {
    pub(crate) sound: PSound,
}

impl Sound {
    pub fn new(sound: PSound) -> Self {
        Self { sound }
    }
    pub fn get_sound(self) -> PSound {
        self.sound
    }
}

impl Object for Sound {
    fn get_type(self: Box<Self>) -> Type {
        Type::Sound(*self)
    }

    fn inspect(&self) -> String {
        format!("Note: {:?}", self.sound)
    }
}

#[derive(Clone, Debug)]
pub struct TimeSignature {
    n: u32,
    dur: u32,
}

impl Object for TimeSignature {
    fn get_type(self: Box<Self>) -> Type {
        Type::TimeSignature(*self)
    }

    fn inspect(&self) -> String {
        "TimeSignature".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Null {}

impl Object for Null {
    fn get_type(self: Box<Self>) -> Type {
        Type::Null
    }

    fn inspect(&self) -> String {
        "Null".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct IntObj {
    pub(crate) value: i32,
}

impl Object for IntObj {
    fn get_type(self: Box<Self>) -> Type {
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
    fn get_type(self: Box<Self>) -> Type {
        Type::Float(self.value)
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct StringObj {
    pub(crate) value: String,
}

impl Object for StringObj {
    fn get_type(self: Box<Self>) -> Type {
        Type::String(self.value)
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

pub trait BuiltinFn: Fn(Vec<Box<dyn Object>>) -> Box<dyn Object> + Sync {}
impl BuiltinFn for fn(Vec<Box<dyn Object>>) -> Box<dyn Object> {}
type DefaultBuiltinFunc = fn(Vec<Box<dyn Object + 'static>>) -> Box<dyn Object>;

#[derive(Clone, Debug)]
pub struct BuiltinObj {
    pub(crate) value: DefaultBuiltinFunc,
}

impl Object for BuiltinObj {
    fn get_type(self: Box<Self>) -> Type {
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
    fn get_type(self: Box<Self>) -> Type {
        Type::Error(self.msg)
    }
    fn inspect(&self) -> String {
        format!("ERROR: {}", self.msg)
    }
    fn is_error(&self) -> bool {
        true
    }
}

pub struct Env {
    store: HashMap<String, Box<dyn Object>>,
    outer: Option<Box<Env>>,
}

#[allow(dead_code)]
impl Env {
    pub fn new_enclosed(outer: Env) -> Self {
        Self {
            store: Default::default(),
            outer: Some(Box::new(outer)),
        }
    }
    pub fn new() -> Self {
        Self {
            store: Default::default(),
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<Box<dyn Object>> {
        if let Some(s) = self.store.get(name) {
            return Some(s.clone());
        }
        if let Some(outer) = &self.outer {
            if let Some(outer_res) = outer.store.get(name) {
                return Some(outer_res.clone());
            }
        }
        None
    }

    pub fn set(&mut self, name: String, obj: Box<dyn Object>) {
        self.store.insert(name, obj);
    }
}
