use crate::interpreter::ast::NodeType;
use crate::interpreter::ast::{
    CallExpression, Expression, Identifier, InfixExpression, IntegerLiteral, PrefixExpression,
    Program,
};
use crate::interpreter::lexer::Lexer;
use crate::interpreter::token::{Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::num::ParseIntError;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum Precedence {
    Lowest = 1,
    Sum, // +
    Product,
    Prefix,
    Call, // myFunction(X)
}

lazy_static! {
    static ref PRECEDENCES: HashMap<TokenType, Precedence> = {
        let mut hm = HashMap::new();
        hm.insert(TokenType::Plus, Precedence::Sum);
        hm.insert(TokenType::Minus, Precedence::Sum);
        hm.insert(TokenType::Slash, Precedence::Product);
        hm.insert(TokenType::Asterisk, Precedence::Product);
        hm.insert(TokenType::Lparen, Precedence::Call);
        hm
    };
}

#[derive(Debug)]
pub enum ParseErr {
    NoPrefix(Token),
    NoInfix(Token),
    Peek(TokenType, String),
    IntConv(ParseIntError),
}

pub struct Parser {
    lex: Lexer,
    errors: Vec<ParseErr>,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lex: Lexer) -> Self {
        let mut parser = Self {
            lex,
            errors: vec![],
            cur_token: Default::default(),
            peek_token: Default::default(),
        };
        //read first two tokens so cur_token and peek_token are set
        parser.next_token();
        parser.next_token();
        parser
    }

    #[allow(dead_code)]
    pub fn get_errors(&self) -> &Vec<ParseErr> {
        &self.errors
    }

    fn parse_whole_expr(&mut self) -> Option<Box<dyn Expression>> {
        let expr = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        expr
    }

    pub fn parse_program(&mut self) -> Program {
        let mut exprs = vec![];

        while !self.cur_token_is(TokenType::Eof) {
            if let Some(e) = self.parse_whole_expr() {
                exprs.push(e);
            }
            self.next_token();
        }
        Program { exprs }
    }

    fn parse_plus_infix(&mut self, left: Box<dyn Expression>) -> Box<dyn Expression> {
        let precedence = self.cur_precedence();
        let tok = self.cur_token.clone();
        self.next_token();

        Box::new(InfixExpression {
            token: tok.clone(),
            left,
            operator: tok.literal,
            right: self.parse_expression(precedence),
        })
    }

    fn parse_call_exp(&mut self, func: Box<dyn Expression>) -> Box<dyn Expression> {
        let token = self.cur_token.clone();
        let args = self.parse_expr_list(TokenType::Rparen);
        Box::new(CallExpression { token, func, args })
    }

    fn parse_expr_list(&mut self, end: TokenType) -> Vec<Box<dyn Expression>> {
        let mut args = vec![];

        if self.peek_token_is(end) {
            self.next_token();
            return args;
        }
        self.next_token();

        if let Some(ex) = self.parse_expression(Precedence::Lowest) {
            args.push(ex);
        }

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            if let Some(ex) = self.parse_expression(Precedence::Lowest) {
                args.push(ex);
            }
        }

        if self.expect_peek(end).is_none() {
            return vec![];
        }
        args
    }

    fn infix(
        &mut self,
        token_type: TokenType,
        expr: Box<dyn Expression>,
    ) -> Option<Box<dyn Expression>> {
        match token_type {
            TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash => {
                self.next_token();
                Some(self.parse_plus_infix(expr))
            }
            TokenType::Lparen => {
                self.next_token();
                Some(self.parse_call_exp(expr))
            }
            _ => unimplemented!("{:?}", token_type),
        }
    }

    fn prefix(&mut self, token_type: TokenType) -> Option<Box<dyn Expression>> {
        match token_type {
            TokenType::Lparen => self.parse_grouped_expr(),
            TokenType::Int => self.parse_int_lit(),
            TokenType::Minus | TokenType::Asterisk => self.parse_prefix_expr(),
            TokenType::Ident => Some(self.parse_ident()),
            _ => unimplemented!("prefix {:?}", self.cur_token),
        }
    }

    fn parse_ident(&self) -> Box<dyn Expression> {
        let tok = self.cur_token.clone();
        let lit = tok.literal.to_string();
        Box::new(Identifier {
            token: tok,
            value: lit,
        })
    }

    fn parse_prefix_expr(&mut self) -> Option<Box<dyn Expression>> {
        let cur_token = self.cur_token.clone();
        self.next_token();

        let right = self.parse_expression(Precedence::Prefix)?;
        Some(Box::new(PrefixExpression {
            token: cur_token.clone(),
            operator: cur_token.literal,
            right,
        }))
    }

    fn parse_int_lit(&mut self) -> Option<Box<dyn Expression>> {
        let value: i32 = match self.cur_token.literal.parse() {
            Ok(i) => i,
            Err(e) => {
                self.errors.push(ParseErr::IntConv(e));
                return None;
            }
        };
        Some(Box::new(IntegerLiteral {
            token: self.cur_token.clone(),
            value,
        }))
    }

    fn parse_grouped_expr(&mut self) -> Option<Box<dyn Expression>> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest);
        self.expect_peek(TokenType::Rparen)?;
        exp
    }

    fn expect_peek(&mut self, token_type: TokenType) -> Option<()> {
        if self.peek_token_is(token_type) {
            self.next_token();
            return Some(());
        }
        self.peek_err(token_type);
        None
    }

    fn peek_err(&mut self, token_type: TokenType) {
        self.errors.push(ParseErr::Peek(
            token_type,
            format!(
                "expected next token to be {:?}, got {:?} instead",
                token_type, self.peek_token.ttype
            ),
        ))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let mut left_exp = match self.prefix(self.cur_token.ttype) {
            Some(pr) => pr,
            None => {
                self.errors.push(ParseErr::NoPrefix(self.cur_token.clone()));
                return None;
            }
        };

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            left_exp = match self.infix(self.peek_token.ttype, left_exp.clone()) {
                Some(le) => le,
                None => {
                    self.errors.push(ParseErr::NoInfix(self.cur_token.clone()));
                    return Some(left_exp);
                }
            };
        }
        Some(left_exp)
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.ttype == token_type
    }
    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.ttype == token_type
    }
    fn peek_precedence(&self) -> Precedence {
        *PRECEDENCES
            .get(&self.peek_token.ttype)
            .unwrap_or(&Precedence::Lowest)
    }

    fn cur_precedence(&self) -> Precedence {
        *PRECEDENCES
            .get(&self.cur_token.ttype)
            .unwrap_or(&Precedence::Lowest)
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }
}

#[test]
fn test_call_expression_parsing() {
    let input = "play(c#_1_4*, c#_4_4);";

    let lex = Lexer::new(input);
    let mut p = Parser::new(lex);
    let prog = p.parse_program();
    assert_eq!(1, prog.exprs.len());
    let expr = prog.exprs.first().unwrap();

    if let NodeType::CallExp(ce) = expr.clone().get_type() {
        assert_eq!(2, ce.args.len());
        let mut arg_iter = ce.args.iter();
        let arg = arg_iter.next().unwrap();

        if let NodeType::Ident(ident) = arg.clone().get_type() {
            assert_eq!("c#_1_4*", ident.value);
        }

        let arg = arg_iter.next().unwrap();

        if let NodeType::Ident(ident) = arg.clone().get_type() {
            assert_eq!("c#_4_4", ident.value);
            return;
        }
        panic!("expected ident arg got {:?}", arg);
    }
    panic!("expected call expression got {:?}", expr);
}
