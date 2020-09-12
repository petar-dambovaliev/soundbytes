use crate::interpreter::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    pub input: Vec<char>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    pub(crate) ch: char,  // current char under examination
    pub(crate) line: usize,
}

const DEFAULT_CHAR: char = '\x00';

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let mut lex = Self {
            input: chars,
            position: 0,
            read_position: 0,
            ch: DEFAULT_CHAR,
            line: 0,
        };
        lex.read_char();
        lex
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_all_whitespace();
        self.skip_comments();
        self.skip_all_whitespace();
        let tok;

        match self.ch {
            '+' => tok = new_token(TokenType::Plus, self.ch, self.line),
            '*' => tok = new_token(TokenType::Asterisk, self.ch, self.line),
            '(' => tok = new_token(TokenType::Lparen, self.ch, self.line),
            ')' => tok = new_token(TokenType::Rparen, self.ch, self.line),
            ';' => tok = new_token(TokenType::Semicolon, self.ch, self.line),
            '/' => tok = new_token(TokenType::Slash, self.ch, self.line),
            '-' => tok = new_token(TokenType::Minus, self.ch, self.line),
            ',' => tok = new_token(TokenType::Comma, self.ch, self.line),
            '=' => tok = new_token(TokenType::Assign, self.ch, self.line),
            DEFAULT_CHAR => tok = new_token(TokenType::Eof, DEFAULT_CHAR, self.line),
            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_ident();

                    return Token {
                        ttype: lookup_ident(&literal),
                        literal,
                        line: self.line,
                    };
                }
                if self.ch.is_digit(10) {
                    return Token {
                        ttype: TokenType::Int,
                        literal: self.read_number(),
                        line: self.line,
                    };
                }
                tok = Token {
                    ttype: TokenType::Illegal,
                    literal: self.ch.to_string(),
                    line: self.line,
                };
            }
        }
        self.read_char();
        tok
    }
    #[allow(dead_code)]
    fn read_string(&mut self) -> String {
        let position = self.position + 1;

        loop {
            self.read_char();
            if self.ch == '"' || self.ch == DEFAULT_CHAR {
                break;
            }
        }

        self.input[position..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while self.ch.is_digit(10) {
            self.read_char()
        }

        self.input[position..self.position].iter().collect()
    }

    fn read_ident(&mut self) -> String {
        let position = self.position;

        while self.ch.is_alphabetic() || self.is_ident_char() {
            self.read_char()
        }

        self.input[position..self.position].iter().collect()
    }

    fn is_ident_char(&self) -> bool {
        self.ch == '#' || self.ch == '*' || self.ch == '_' || self.ch.is_digit(10)
    }

    fn peek_char(&mut self) -> char {
        match self.input.get(self.read_position) {
            Some(&ch) => ch,
            None => DEFAULT_CHAR,
        }
    }

    fn skip_comments(&mut self) {
        if self.ch == '/' && self.peek_char() == '/' {
            while self.ch != '\n' && self.ch != DEFAULT_CHAR {
                self.read_char();
            }
            self.skip_all_whitespace();
            self.skip_comments();
        }
    }

    fn skip_all_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        self.ch = match self.input.get(self.read_position) {
            Some(&c) => c,
            None => DEFAULT_CHAR,
        };

        if self.ch.to_string() == ENTER {
            self.line += 1;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}
#[allow(dead_code)]
const CR: &str = "\r";
#[allow(dead_code)]
const LF: &str = "\n";
#[allow(dead_code)]
const CRLF: &str = "\r\n";

/// newlines' character is CRLF. This indicate you're using Windows OS.
#[cfg(target_os = "windows")]
pub const ENTER: &str = CRLF;

/// newlines' character is LF. This indicate you're using non-Windows OS.
#[cfg(not(target_os = "windows"))]
pub const ENTER: &str = LF;

fn new_token(ttype: TokenType, ch: char, line: usize) -> Token {
    Token {
        ttype,
        literal: ch.to_string(),
        line,
    }
}

#[test]
fn test_next_token() {
    let input = "tempo(66);1+2;play(c#);play(c#_1_4);";
    let tokens_type: [TokenType; 19] = [
        TokenType::Ident,
        TokenType::Lparen,
        TokenType::Int,
        TokenType::Rparen,
        TokenType::Semicolon,
        TokenType::Int,
        TokenType::Plus,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Ident,
        TokenType::Lparen,
        TokenType::Ident,
        TokenType::Rparen,
        TokenType::Semicolon,
        TokenType::Ident,
        TokenType::Lparen,
        TokenType::Ident,
        TokenType::Rparen,
        TokenType::Semicolon,
    ];
    let tokens_str: [&str; 19] = [
        "tempo", "(", "66", ")", ";", "1", "+", "2", ";", "play", "(", "c#", ")", ";", "play", "(",
        "c#_1_4", ")", ";",
    ];

    let mut lex = Lexer::new(input);

    for (key, token) in tokens_type.iter().enumerate() {
        let tok = lex.next_token();
        assert_eq!(token, &tok.ttype);
        assert_eq!(tokens_str[key], tok.literal);
    }
}

#[test]
fn test_next_token_play() {
    let input = "play(c#_1_4);";
    let tokens_type: [TokenType; 5] = [
        TokenType::Ident,
        TokenType::Lparen,
        TokenType::Ident,
        TokenType::Rparen,
        TokenType::Semicolon,
    ];
    let tokens_str: [&str; 5] = ["play", "(", "c#_1_4", ")", ";"];

    let mut lex = Lexer::new(input);

    for (key, token) in tokens_type.iter().enumerate() {
        let tok = lex.next_token();
        assert_eq!(token, &tok.ttype);
        assert_eq!(tokens_str[key], tok.literal);
    }
}

#[test]
fn test_multiline_expression() {
    let input = "play(c#_1_4,\n c#_1_4);";
    let tokens_type: [TokenType; 7] = [
        TokenType::Ident,
        TokenType::Lparen,
        TokenType::Ident,
        TokenType::Comma,
        TokenType::Ident,
        TokenType::Rparen,
        TokenType::Semicolon,
    ];
    let tokens_str: [&str; 7] = ["play", "(", "c#_1_4", ",", "c#_1_4", ")", ";"];

    let mut lex = Lexer::new(input);

    for (key, token) in tokens_type.iter().enumerate() {
        let tok = lex.next_token();
        assert_eq!(token, &tok.ttype);
        assert_eq!(tokens_str[key], tok.literal);
    }
}

#[test]
fn test_skip_comments() {
    let input = "
tempo(40);

//make multiline work
// set first octave and duration as defaults
// make multiple tracks playable simultaneously
// write intellij plugin
// imeplement + operator for notes



play(a_5_32,
a_4_32);";
    let mut lex = Lexer::new(input);

    let tok = lex.next_token();
    assert_eq!(TokenType::Ident, tok.ttype);
    assert_eq!("tempo", tok.literal);
    assert_eq!(1, tok.line);

    let tok = lex.next_token();
    assert_eq!(TokenType::Lparen, tok.ttype);
    assert_eq!(1, tok.line);

    let tok = lex.next_token();
    assert_eq!(TokenType::Int, tok.ttype);
    assert_eq!("40", tok.literal);
    assert_eq!(1, tok.line);

    let tok = lex.next_token();
    assert_eq!(TokenType::Rparen, tok.ttype);
    assert_eq!(1, tok.line);

    let tok = lex.next_token();
    assert_eq!(TokenType::Semicolon, tok.ttype);
    assert_eq!(1, tok.line);

    let tok = lex.next_token();
    assert_eq!(TokenType::Ident, tok.ttype);
    assert_eq!("play", tok.literal);
    assert_eq!(11, tok.line);
}

#[test]
fn test_assignment() {
    let input = "let foo = c_4_4;";
    let tokens_type: [TokenType; 5] = [
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Ident,
        TokenType::Semicolon,
    ];
    let tokens_str: [&str; 5] = ["let", "foo", "=", "c_4_4", ";"];

    let mut lex = Lexer::new(input);

    for (key, token) in tokens_type.iter().enumerate() {
        let tok = lex.next_token();
        assert_eq!(token, &tok.ttype);
        assert_eq!(tokens_str[key], tok.literal);
    }
}
