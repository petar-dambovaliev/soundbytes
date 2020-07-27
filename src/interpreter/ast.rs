use crate::interpreter::token::Token;
use std::fmt::{Debug, Write};

// The base Node interface
pub trait Node {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
    fn get_type(self: Box<Self>) -> NodeType;
}

pub enum NodeType {
    CallExp(Box<CallExpression>),
    InfixExp(Box<InfixExpression>),
    IntLit(Box<IntegerLiteral>),
    Ident(Box<Identifier>),
}

// All expression nodes implement this
pub trait Expression: Node + CloneExp + Debug {
    fn expression_node(&self);
    fn to_node(&self) -> Box<dyn Node>;
}

pub trait CloneExp {
    fn clone_exp(&self) -> Box<dyn Expression>;
}

impl<T> CloneExp for T
where
    T: Expression + Node + Clone + 'static,
{
    fn clone_exp(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Self {
        self.clone_exp()
    }
}

#[derive(Clone, Debug)]
pub struct Identifier {
    token: Token, // the token.IDENT token
    value: String,
}

impl Identifier {
    pub fn get_value(&self) -> String {
        self.value.to_string()
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn get_type(self: Box<Self>) -> NodeType {
        NodeType::Ident(self)
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}

    fn to_node(&self) -> Box<dyn Node> {
        let node: Box<dyn Node> = Box::new(self.clone());
        node
    }
}

#[derive(Clone, Debug)]
pub struct CallExpression {
    pub(crate) token: Token,              // The '(' token
    pub(crate) func: Box<dyn Expression>, // Identifier or FunctionLiteral
    pub(crate) args: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        let mut args = vec![];

        for arg in &self.args {
            args.push(arg.to_string());
        }

        let _ = out.write_str(&self.func.to_string());
        let _ = out.write_char('(');
        let _ = out.write_str(&args.join(", "));
        let _ = out.write_char(')');
        out
    }

    fn get_type(self: Box<Self>) -> NodeType {
        NodeType::CallExp(self)
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}

    fn to_node(&self) -> Box<dyn Node> {
        let node: Box<dyn Node> = Box::new(self.clone());
        node
    }
}

#[derive(Clone, Debug)]
pub struct InfixExpression {
    pub(crate) token: Token, // The operator token, e.g. +
    pub(crate) left: Box<dyn Expression>,
    pub(crate) operator: String,
    pub(crate) right: Option<Box<dyn Expression>>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        let _ = out.write_char('(');
        let _ = out.write_str(&self.left.to_string());
        let _ = out.write_char(' ');
        let _ = out.write_str(&self.operator.to_string());
        let _ = out.write_char(' ');
        let _ = out.write_str(&self.right.as_ref().unwrap().to_string());
        let _ = out.write_char(')');
        out
    }

    fn get_type(self: Box<Self>) -> NodeType {
        NodeType::InfixExp(self)
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}

    fn to_node(&self) -> Box<dyn Node> {
        let node: Box<dyn Node> = Box::new(self.clone());
        node
    }
}

#[derive(Clone, Debug)]
pub struct IntegerLiteral {
    token: Token,
    value: i32,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn to_string(&self) -> String {
        self.token.literal.to_string()
    }

    fn get_type(self: Box<Self>) -> NodeType {
        NodeType::IntLit(self)
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}

    fn to_node(&self) -> Box<dyn Node> {
        let node: Box<dyn Node> = Box::new(self.clone());
        node
    }
}
