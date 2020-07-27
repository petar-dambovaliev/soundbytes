use crate::player::instrument::InstrumentBox;
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
    Chord(Chord),
    Instrument(Instrument),
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
            Self::Chord(c) => f.write_str(&format!("chord {:?}", c)),
            Self::Instrument(n) => f.write_str(&n.inspect()),
            Self::Note(n) => f.write_str(&n.inspect()),
            Self::Octave(n) => f.write_str(&n.inspect()),
            Self::Duration(n) => f.write_str(&n.inspect()),
            Self::Null => f.write_str("Null"),
        }
    }
}

type ObjectBox = Box<dyn Object>;

pub trait Object: CloneObj + Debug {
    fn get_type(self: Box<Self>) -> Type;
    fn inspect(&self) -> String;
    fn is_error(&self) -> bool {
        false
    }
}

pub trait CloneObj {
    fn clone_obj(&self) -> ObjectBox;
}

impl<T> CloneObj for T
where
    T: Object + Clone + 'static,
{
    fn clone_obj(&self) -> ObjectBox {
        Box::new(self.clone())
    }
}

impl Clone for ObjectBox {
    fn clone(&self) -> Self {
        self.clone_obj()
    }
}

#[derive(Clone, Debug)]
pub struct Chord {
    sounds: Vec<Sound>,
}

impl Chord {
    pub fn new(sounds: Vec<Sound>) -> Self {
        Self { sounds }
    }
    pub fn get_sounds(self) -> Vec<Sound> {
        self.sounds
    }
}

impl Object for Chord {
    fn get_type(self: Box<Self>) -> Type {
        Type::Chord(*self)
    }

    fn inspect(&self) -> String {
        format!("chord: {:?}", self.sounds)
    }
}

#[derive(Clone, Debug)]
pub struct Instrument {
    ins: InstrumentBox,
}

impl Instrument {
    pub fn new(ins: InstrumentBox) -> Self {
        Self { ins }
    }
    pub fn get_instrument(&self) -> InstrumentBox {
        self.ins.clone()
    }
}

impl Object for Instrument {
    fn get_type(self: Box<Self>) -> Type {
        Type::Instrument(*self)
    }

    fn inspect(&self) -> String {
        format!("Instrument: {:?}", self.ins)
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
        format!("Duration: {:?}", self.dur)
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
        format!("Octave: {:?}", self.octave)
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
    pub(crate) modified: bool,
}

impl Sound {
    pub fn new(sound: PSound, modified: bool) -> Self {
        Self { sound, modified }
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
        format!("Object Sound: {:?}", self)
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

pub trait BuiltinFn: Fn(Vec<ObjectBox>) -> ObjectBox + Sync {}
impl BuiltinFn for fn(Vec<ObjectBox>) -> ObjectBox {}
type DefaultBuiltinFunc = fn(Vec<Box<dyn Object + 'static>>) -> ObjectBox;

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

#[derive(Debug)]
pub struct Env {
    store: HashMap<String, ObjectBox>,
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

    pub fn get(&self, name: &str) -> Option<ObjectBox> {
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

    pub fn set(&mut self, name: String, obj: ObjectBox) {
        self.store.insert(name, obj);
    }
}
