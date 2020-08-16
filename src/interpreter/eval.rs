use crate::interpreter::ast::{
    CallExpression, Expression, Identifier, InfixExpression, Node, NodeType,
};
use crate::interpreter::builtin::BUILTINS;
use crate::interpreter::object::{CloneObj, Env, Error, IntObj, Object, Type};

pub fn eval(node: Box<dyn Node>, env: &Env) -> Box<dyn Object> {
    match node.get_type() {
        NodeType::CallExp(call_exp) => eval_call_exp(*call_exp, env),
        NodeType::InfixExp(infix_exp) => eval_infix_expr(&infix_exp, env),
        NodeType::Ident(ident) => eval_ident(ident, env),
        NodeType::IntLit(int_lit) => {
            let int_obj = IntObj {
                value: int_lit.get_value(),
            };
            int_obj.clone_obj()
        }
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

fn eval_infix_expr(infix_exp: &InfixExpression, env: &Env) -> Box<dyn Object> {
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
                "unknown operator: op - {}  left - {}  right - {}",
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

fn eval_ident(ident: Box<Identifier>, env: &Env) -> Box<dyn Object> {
    if let Some(val) = env.get(ident.get_value().as_str()) {
        return val.clone_obj();
    }

    if let Some(builtin) = BUILTINS.get(ident.get_value().as_str()) {
        return builtin.clone_obj();
    }

    new_error(format!("identifier not found: {:?}", ident))
}
