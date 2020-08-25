use crate::interpreter::ast::{
    CallExpression, Expression, Identifier, InfixExpression, Node, NodeType, PrefixExpression,
};
use crate::interpreter::builtin::BUILTINS;
use crate::interpreter::object::{
    CloneObj, Duration, Env, Error, IntObj, Note, Object, Octave, Sound, Type,
};
use crate::player::sound::{Note as PNote, Sound as PSound};
use crate::player::tempo::Duration as PDuration;

pub fn eval(node: Box<dyn Node>, env: &Env) -> Box<dyn Object> {
    match node.get_type() {
        NodeType::CallExp(call_exp) => eval_call_exp(*call_exp, env),
        NodeType::InfixExp(infix_exp) => eval_infix_expr(*infix_exp, env),
        NodeType::Ident(ident) => eval_ident(ident, env),
        NodeType::IntLit(int_lit) => {
            let int_obj = IntObj {
                value: int_lit.get_value(),
            };
            int_obj.clone_obj()
        }
        NodeType::PrefixExpr(prefix_expr) => eval_prefix_expr(*prefix_expr, env),
    }
}

fn eval_prefix_expr(prefix_exp: PrefixExpression, env: &Env) -> Box<dyn Object> {
    let right = eval(prefix_exp.right.to_node(), env);

    if right.is_error() {
        return right;
    }

    match prefix_exp.operator.as_str() {
        "-" => eval_minus_prefix(right),
        _ => new_error(format!(
            "unknown operator: '{}' {:?}",
            prefix_exp.operator,
            right.get_type()
        )),
    }
}

fn eval_minus_prefix(right: Box<dyn Object>) -> Box<dyn Object> {
    let t = right.get_type();
    match t {
        Type::Int(i) => Box::new(IntObj { value: -i }),
        _ => new_error(format!("unknown operator: -{:?}", t)),
    }
}

fn eval_call_exp(call_exp: CallExpression, env: &Env) -> Box<dyn Object> {
    let func = eval(call_exp.func.to_node(), env);
    if func.is_error() {
        return func;
    }

    let args = eval_exprs(call_exp.args, env);
    if let Some(arg) = args.get(0) {
        if arg.is_error() {
            return arg.clone_obj();
        }
    }

    let ins = func.inspect();
    if let Type::Builtin(function) = func.get_type() {
        return function(args);
    }
    new_error(format!("not a function: {}", ins))
}

pub fn new_error(msg: String) -> Box<dyn Object> {
    let err: Box<dyn Object> = Box::new(Error { msg });
    err
}

fn eval_infix_expr(infix_exp: InfixExpression, env: &Env) -> Box<dyn Object> {
    let left = eval(infix_exp.left.to_node(), env);
    if left.is_error() {
        return left;
    }

    let right = eval(infix_exp.right.as_ref().unwrap().to_node(), env);
    if right.is_error() {
        return right;
    }

    let left_ins = left.inspect();
    let right_ins = right.inspect();

    if let (Type::Int(l), Type::Int(r)) = (left.get_type(), right.get_type()) {
        return eval_int_infix_expr(&infix_exp.operator, l, r);
    }
    new_error(format!(
        "unknown operator: {:?} {:?} {:?}",
        left_ins, infix_exp.operator, right_ins
    ))
}

fn eval_int_infix_expr(op: &str, left: i32, right: i32) -> Box<dyn Object> {
    let int = match op {
        "+" => left + right,
        "-" => left - right,
        "/" => left / right,
        "*" => left * right,
        _ => {
            return new_error(format!(
                "unknown operator: op: '{}'  left: '{}'  right: '{}'",
                op, left, right
            ))
        }
    };
    let obj: Box<dyn Object> = Box::new(IntObj { value: int });
    obj
}

fn eval_exprs(expr: Vec<Box<dyn Expression>>, env: &Env) -> Vec<Box<dyn Object>> {
    let mut objs = vec![];

    for ex in expr {
        let evaluated = eval(ex.to_node(), env);
        if evaluated.is_error() {
            return vec![evaluated];
        }
        objs.push(evaluated);
    }
    objs
}

fn eval_note_ident(ident: Identifier, env: &Env) -> Box<dyn Object> {
    let ident_val = ident.get_value();
    let mut spl = ident_val.split('_');

    let n = match spl.next() {
        Some(note) => {
            let n_eval = eval_ident(
                Box::new(Identifier {
                    token: Token {
                        ttype: TokenType::Ident,
                        literal: note.to_string(),
                    },
                    value: note.to_string(),
                }),
                env,
            );

            match n_eval.get_type() {
                Type::Note(note) => note,
                _ => return new_error("invalid note".to_string()),
            }
        }
        _ => return new_error("invalid note".to_string()),
    };

    let oct = match spl.next() {
        Some(o) => {
            let oct_eval = eval_ident(
                Box::new(Identifier {
                    token: Token {
                        ttype: TokenType::Ident,
                        literal: format!("o{}", o),
                    },
                    value: format!("o{}", o),
                }),
                env,
            );

            match oct_eval.get_type() {
                Type::Octave(o) => o,
                _ => return new_error("invalid note arg 2 octave".to_string()),
            }
        }
        _ => return new_error("invalid note arg 2 octave".to_string()),
    };

    let dur = match spl.next() {
        Some(d) => {
            let dur_eval = eval_ident(
                Box::new(Identifier {
                    token: Token {
                        ttype: TokenType::Ident,
                        literal: format!("d{}", d),
                    },
                    value: format!("d{}", d),
                }),
                env,
            );
            match dur_eval.get_type() {
                Type::Duration(d) => d,
                _ => return new_error("invalid note arg 3 duration".to_string()),
            }
        }
        _ => return new_error("invalid note arg 3 duration".to_string()),
    };

    Box::new(Sound {
        sound: PSound {
            note: n.get_note(),
            octave: oct.get_oct(),
            duration: dur.get_dur(),
            effects: None,
        },
    })
}

fn eval_ident(ident: Box<Identifier>, env: &Env) -> Box<dyn Object> {
    if let Some(val) = env.get(ident.get_value().as_str()) {
        return val.clone_obj();
    }

    if let Some(builtin) = BUILTINS.get(ident.get_value().as_str()) {
        return builtin.clone_obj();
    }

    if ident.get_value().contains('_') {
        return eval_note_ident(*ident, env);
    }

    new_error(format!("identifier not found: {:?}", ident))
}

#[cfg(test)]
use crate::interpreter::lexer::Lexer;
#[cfg(test)]
use crate::interpreter::parser::Parser;
use crate::interpreter::token::{Token, TokenType};
use std::collections::VecDeque;

#[test]
fn test_eval_int_expr() {
    let tests = vec![
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("5 + 5 - 10 + 5", 5),
        ("2 * 2", 4),
        ("10/5", 2),
    ];

    for (expr, res) in tests {
        let lex = Lexer::new(expr);
        let mut p = Parser::new(lex);
        let program = p.parse_program();
        for exp in program.exprs {
            let env = Env::new();
            let evaluated = eval(exp.to_node(), &env);
            let t = evaluated.get_type();
            match &t {
                Type::Int(i) => assert_eq!(&res, i),
                _ => panic!("expected Int, got {:?}", t),
            }
        }
    }
}

#[test]
#[should_panic(expected = "not implemented: prefix Token { ttype: Illegal, literal: \".\" }")]
fn test_eval_float_not_implemented_expr() {
    let expr = "5.5";

    let lex = Lexer::new(expr);
    let mut p = Parser::new(lex);
    let program = p.parse_program();
    for exp in program.exprs {
        let env = Env::new();
        let _ = eval(exp.to_node(), &env);
    }
}

#[test]
fn test_eval_play() {
    let tests = vec!["play(c#_4_4);"];

    for expr in tests {
        let lex = Lexer::new(expr);
        let mut p = Parser::new(lex);
        let program = p.parse_program();
        for exp in program.exprs {
            let env = Env::new();
            let evaluated = eval(exp.to_node(), &env);
            let t = evaluated.get_type();
            println!("{:?}", t);
        }
    }
}
