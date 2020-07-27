use lazy_static::lazy_static;
use std::collections::HashMap;

pub fn lookup_ident(ident: &str) -> TokenType {
    *KEYWORDS.get(ident).unwrap_or(&TokenType::Ident)
}

#[derive(Default, Clone, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifier
    Ident,
    Lt,
    Gt,

    // Literal
    Int,

    // Operators
    Plus,
    Asterisk,

    Eq,
    NotEq,

    // Delimiter
    Comma,
    Semicolon,
    Colon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    // Keyword
    Play,
    Repeat,
    If,
    Tempo,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Illegal
    }
}

impl TokenType {
    fn from_str(s: &str) -> Self {
        match s {
            "ILLEGAL" => Self::Illegal,
            "EOF" => Self::Eof,
            "IDENT" => Self::Ident,
            "INT" => Self::Int,
            "+" => Self::Plus,
            "*" => Self::Asterisk,
            "," => Self::Comma,
            ";" => Self::Semicolon,
            "(" => Self::Lparen,
            ")" => Self::Rparen,
            "{" => Self::Lbrace,
            "}" => Self::Rbrace,
            "PLAY" => Self::Play,
            "REPEAT" => Self::Repeat,
            "IF" => Self::If,
            "TEMPO" => Self::Tempo,
            _ => Self::Illegal,
        }
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut hp = HashMap::new();
        hp.insert("play".to_string(), TokenType::Play);
        hp.insert("repeat".to_string(), TokenType::Repeat);
        hp.insert("tempo".to_string(), TokenType::Tempo);
        hp
    };
}
