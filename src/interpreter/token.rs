use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn lookup_ident(ident: &str) -> TokenType {
    *KEYWORDS.get(ident).unwrap_or(&TokenType::Ident)
}

#[derive(Default, Clone, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
#[allow(dead_code)]
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
    Slash,
    Minus,

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
    //Play,
    //Repeat,
    If,
    Assign,
    Let,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Illegal
    }
}

#[allow(dead_code)]
impl TokenType {
    fn from_str(s: &str) -> Self {
        match s {
            "eof" => Self::Eof,
            "ident" => Self::Ident,
            "int" => Self::Int,
            "+" => Self::Plus,
            "*" => Self::Asterisk,
            "," => Self::Comma,
            ";" => Self::Semicolon,
            "(" => Self::Lparen,
            ")" => Self::Rparen,
            "{" => Self::Lbrace,
            "}" => Self::Rbrace,
            "/" => Self::Slash,
            "-" => Self::Minus,
            "=" => Self::Assign,
            "let" => Self::Let,
            //"REPEAT" => Self::Repeat,
            "IF" => Self::If,
            //"TEMPO" => Self::Tempo,
            _ => Self::Illegal,
        }
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut hp = HashMap::new();
        hp.insert("let".to_string(), TokenType::Let);
        // hp.insert("repeat".to_string(), TokenType::Repeat);
        // hp.insert("tempo".to_string(), TokenType::Tempo);
        hp
    };
}
