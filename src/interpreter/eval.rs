use crate::interpreter::ast::{
    CallExpression, Expression, Identifier, InfixExpression, Node, NodeType,
};
use crate::interpreter::object::Type::Builtin;
use crate::interpreter::object::{
    BuiltinObj, Env, Error, FloatObj, IntObj, Object, StringObj, Type,
};
use std::option::Option::Some;

pub fn eval(node: Box<dyn Node>, env: &Env) -> Box<dyn Object> {
    match node.get_type() {
        NodeType::CallExp(call_exp) => eval_call_exp(call_exp, env),
        NodeType::InfixExp(infix_exp) => eval_infix_expr(infix_exp, env),
        NodeType::Ident(ident) => eval_ident(ident, env),
        NodeType::IntLit(int_lit) => {}
    }
}

fn eval_call_exp(call_exp: Box<CallExpression>, env: &Env) -> Box<dyn Object> {
    let func = eval(call_exp.func.to_node(), env);
    if func.is_error() {
        return func;
    }

    let args = eval_exprs(call_exp.args, env);
    if let Some(&arg) = args.get(0) {
        if arg.is_error() {
            return arg;
        }
    }

    if let Type::Builtin(function) = func.get_type() {
        return function(args);
    }
    new_error(format!("not a function: {:?}", func.get_type()))
}

pub fn new_error(msg: String) -> Box<dyn Object> {
    let err: Box<dyn Object> = Box::new(Error { msg });
    err
}

fn eval_infix_expr(infix_exp: Box<InfixExpression>, env: &Env) -> Box<dyn Object> {
    let left = eval(infix_exp.left.to_node(), env);
    if left.is_error() {
        return left;
    }

    let right = eval(infix_exp.right.unwrap().to_node(), env);
    if right.is_error() {
        return right;
    }

    if let (Type::Int(l), Type::Int(r)) = (left.get_type(), right.get_type()) {
        return eval_int_infix_expr(&infix_exp.operator, l, r);
    }
    new_error(format!(
        "unknown operator: {:?} {:?} {:?}",
        left.get_type(),
        infix_exp.operator,
        right.get_type()
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
        let obj: Box<dyn Object> = match val.get_type() {
            Type::Int(i) => Box::new(IntObj { value: i }),
            Type::Float(f) => Box::new(FloatObj { value: f }),
            Type::String(string) => Box::new(StringObj { value: string }),
            Type::Builtin(func) => Box::new(BuiltinObj { value: func }),
            Type::Error(string) => Box::new(Error { msg: string }),
        };
        return obj;
    }

    if let Some(builtin) = Builtins {}
    //     if builtin, ok := builtins[node.Value]; ok {
    //         return builtin
    // }
    //
    //     return newError("identifier not found: " + node.Value)
    //     }
    unimplemented!()
}

//
// func Eval(node ast.Node, env *object.Environment) object.Object {
// switch node := node.(type) {
//
// // Statements
// case *ast.Program:
// return evalProgram(node, env)
//
// case *ast.BlockStatement:
// return evalBlockStatement(node, env)
//
// case *ast.ExpressionStatement:
// return Eval(node.Expression, env)
//
// case *ast.ReturnStatement:
// val := Eval(node.ReturnValue, env)
// if isError(val) {
// return val
// }
// return &object.ReturnValue{Value: val}
//
// case *ast.LetStatement:
// val := Eval(node.Value, env)
// if isError(val) {
// return val
// }
// env.Set(node.Name.Value, val)
//
// // Expressions
// case *ast.IntegerLiteral:
// return &object.Integer{Value: node.Value}
//
// case *ast.StringLiteral:
// return &object.String{Value: node.Value}
//
// case *ast.Boolean:
// return nativeBoolToBooleanObject(node.Value)
//
// case *ast.PrefixExpression:
// right := Eval(node.Right, env)
// if isError(right) {
// return right
// }
// return evalPrefixExpression(node.Operator, right)
//
// case *ast.InfixExpression:
// left := Eval(node.Left, env)
// if isError(left) {
// return left
// }
//
// right := Eval(node.Right, env)
// if isError(right) {
// return right
// }
//
// return evalInfixExpression(node.Operator, left, right)
//
// case *ast.IfExpression:
// return evalIfExpression(node, env)
//
// case *ast.Identifier:
// return evalIdentifier(node, env)
//
// case *ast.FunctionLiteral:
// params := node.Parameters
// body := node.Body
// return &object.Function{Parameters: params, Env: env, Body: body}
//
// case *ast.CallExpression:
// function := Eval(node.Function, env)
// if isError(function) {
// return function
// }
//
// args := evalExpressions(node.Arguments, env)
// if len(args) == 1 && isError(args[0]) {
// return args[0]
// }
//
// return applyFunction(function, args)
//
// case *ast.ArrayLiteral:
// elements := evalExpressions(node.Elements, env)
// if len(elements) == 1 && isError(elements[0]) {
// return elements[0]
// }
// return &object.Array{Elements: elements}
//
// case *ast.IndexExpression:
// left := Eval(node.Left, env)
// if isError(left) {
// return left
// }
// index := Eval(node.Index, env)
// if isError(index) {
// return index
// }
// return evalIndexExpression(left, index)
//
// case *ast.HashLiteral:
// return evalHashLiteral(node, env)
//
// }
//
// return nil
// }
