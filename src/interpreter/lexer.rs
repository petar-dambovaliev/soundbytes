use crate::interpreter::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    pub input: Vec<char>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    pub(crate) ch: char,  // current char under examination
}

const DEFAULT_CHAR: char = '\x00';

#[allow(dead_code)]
impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let mut lex = Self {
            input: chars,
            position: 0,
            read_position: 0,
            ch: DEFAULT_CHAR,
        };
        lex.read_char();
        lex
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok;

        match self.ch {
            '+' => tok = new_token(TokenType::Plus, self.ch),
            '*' => tok = new_token(TokenType::Asterisk, self.ch),
            '(' => tok = new_token(TokenType::Lparen, self.ch),
            ')' => tok = new_token(TokenType::Rparen, self.ch),
            ';' => tok = new_token(TokenType::Semicolon, self.ch),
            '/' => tok = new_token(TokenType::Slash, self.ch),
            '-' => tok = new_token(TokenType::Minus, self.ch),
            DEFAULT_CHAR => tok = new_token(TokenType::Eof, DEFAULT_CHAR),
            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_ident();

                    return Token {
                        ttype: lookup_ident(literal.as_str()),
                        literal,
                    };
                }
                if self.ch.is_digit(10) {
                    return Token {
                        ttype: TokenType::Int,
                        literal: self.read_number(),
                    };
                }
                tok = Token {
                    ttype: TokenType::Illegal,
                    literal: self.ch.to_string(),
                };
            }
        }
        self.read_char();
        tok
    }

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

        while self.ch.is_alphabetic() {
            self.read_char()
        }

        self.input[position..self.position].iter().collect()
    }

    fn peek_char(&mut self) -> char {
        match self.input.get(self.read_position) {
            Some(&ch) => ch,
            None => DEFAULT_CHAR,
        }
    }

    fn skip_whitespace(&mut self) {
        if self.ch.is_whitespace() {
            self.read_char()
        }
    }

    fn read_char(&mut self) {
        self.ch = match self.input.get(self.read_position) {
            Some(&c) => c,
            None => DEFAULT_CHAR,
        };

        self.position = self.read_position;
        self.read_position += 1;
    }
}

fn new_token(ttype: TokenType, ch: char) -> Token {
    Token {
        ttype,
        literal: ch.to_string(),
    }
}

#[test]
fn test_next_token() {
    let input = "tempo(66);1+2;";
    let tokens_type: [TokenType; 9] = [
        TokenType::Ident,
        TokenType::Lparen,
        TokenType::Int,
        TokenType::Rparen,
        TokenType::Semicolon,
        TokenType::Int,
        TokenType::Plus,
        TokenType::Int,
        TokenType::Semicolon,
    ];
    let tokens_str: [&str; 9] = ["tempo", "(", "66", ")", ";", "1", "+", "2", ";"];

    let mut lex = Lexer::new(input);

    for (key, token) in tokens_type.iter().enumerate() {
        let tok = lex.next_token();
        assert_eq!(token, &tok.ttype);
        assert_eq!(tokens_str[key], tok.literal);
    }
}
