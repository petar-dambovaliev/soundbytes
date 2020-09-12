use crate::interpreter::ast::{
    AssignStatement, CallExpression, Expression, Identifier, InfixExpression, Node, NodeType,
    PrefixExpression,
};
use crate::interpreter::builtin::BUILTINS;
use crate::interpreter::object::{
    Chord, CloneObj, Env, Error, IntObj, Note, Null, Object, Sound, Type,
};
use crate::interpreter::token::{Token, TokenType};
use crate::player::sound::{Note as PNote, Sound as PSound};

pub fn eval(node: Box<dyn Node>, env: &mut Env) -> Box<dyn Object> {
    match node.get_type() {
        NodeType::CallExp(call_exp) => eval_call_exp(*call_exp, env),
        NodeType::InfixExp(infix_exp) => eval_infix_expr(*infix_exp, env),
        NodeType::Ident(ident) => eval_ident(*ident, env),
        NodeType::IntLit(int_lit) => {
            let int_obj = IntObj {
                value: int_lit.get_value(),
            };
            int_obj.clone_obj()
        }
        NodeType::PrefixExpr(prefix_expr) => eval_prefix_expr(*prefix_expr, env),
        NodeType::AssignStmt(assign_statement) => eval_assign_statement(*assign_statement, env),
    }
}

fn eval_assign_statement(assign_statement: AssignStatement, env: &mut Env) -> Box<dyn Object> {
    let expr = eval(assign_statement.value.to_node(), env);
    if expr.is_error() {
        return expr;
    }
    env.set(assign_statement.name.value, expr);
    Box::new(Null {})
}

fn eval_prefix_expr(prefix_exp: PrefixExpression, env: &mut Env) -> Box<dyn Object> {
    let right = eval(prefix_exp.right.to_node(), env);

    if right.is_error() {
        return right;
    }

    match prefix_exp.operator.as_str() {
        "-" => eval_minus_prefix(right, prefix_exp.token.line),
        _ => new_error(
            format!(
                "unknown operator: '{}' {:?}",
                prefix_exp.operator,
                right.get_type()
            ),
            prefix_exp.token.line,
        ),
    }
}

fn eval_minus_prefix(right: Box<dyn Object>, line: usize) -> Box<dyn Object> {
    let t = right.get_type();
    match t {
        Type::Int(i) => Box::new(IntObj { value: -i }),
        _ => new_error(format!("unknown operator: -{:?}", t), line),
    }
}

fn eval_call_exp(call_exp: CallExpression, env: &mut Env) -> Box<dyn Object> {
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
        return function(args, call_exp.token.line);
    }
    new_error(format!("not a function: {}", ins), call_exp.token.line)
}

pub fn new_error(msg: String, line: usize) -> Box<dyn Object> {
    let err: Box<dyn Object> = Box::new(Error { msg, line });
    err
}

fn eval_infix_expr(infix_exp: InfixExpression, env: &mut Env) -> Box<dyn Object> {
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

    match (left.get_type(), right.get_type()) {
        // 5 + 5
        (Type::Int(l), Type::Int(r)) => {
            eval_int_infix_expr(&infix_exp.operator, l, r, infix_exp.token.line)
        }
        // c_4_16 + c_3_8
        (Type::Sound(l), Type::Sound(r)) => {
            eval_sound_infix_expr(&infix_exp.operator, l, r, infix_exp.token.line)
        }
        // let chord = c_4_16 + c_3_8;
        // chord + c_5_16
        (Type::Chord(l), Type::Sound(r)) => eval_chord_infix_expr(
            &infix_exp.operator,
            l,
            Chord::new(vec![r]),
            infix_exp.token.line,
        ),
        // let chord_left = c_4_16 + c_3_8;
        // let chord_right = c_2_16 + c_3_8;
        // chord_left + chord_right
        (Type::Chord(l), Type::Chord(r)) => {
            eval_chord_infix_expr(&infix_exp.operator, l, r, infix_exp.token.line)
        }
        // let chord = c_4_16 + c_3_8;
        // e_4_16 + chord
        (Type::Sound(l), Type::Chord(r)) => eval_chord_infix_expr(
            &infix_exp.operator,
            Chord::new(vec![l]),
            r,
            infix_exp.token.line,
        ),
        // c_4_16 + e
        (Type::Sound(l), Type::Note(r)) => eval_note_infix_expr(
            &infix_exp.operator,
            Chord::new(vec![l]),
            r,
            infix_exp.token.line,
        ),

        // let chord = c_4_16 + c_3_8;
        // chord + e
        (Type::Chord(l), Type::Note(r)) => {
            eval_note_infix_expr(&infix_exp.operator, l, r, infix_exp.token.line)
        }
        _ => new_error(
            format!(
                "unknown operands for plus: left - {:?}  right - {:?}",
                left_ins, right_ins
            ),
            infix_exp.token.line,
        ),
    }
}

fn eval_note_infix_expr(op: &str, left: Chord, right: Note, line: usize) -> Box<dyn Object> {
    let left_ins = left.inspect();
    let chord = match op {
        "+" => {
            let mut sounds = left.get_sounds();
            let mut oct = None;
            let mut dur = None;
            for s in sounds.iter().rev() {
                if !s.modified {
                    oct = Some(s.sound.octave.clone());
                    dur = Some(s.sound.duration.clone());
                    break;
                }
            }

            let (o, d) = match (oct, dur) {
                (Some(o), Some(d)) => (o, d),
                _ => {
                    return new_error(
                        format!(
                            "unknown operator: op: '{}'  left: {:?}  right: {:?}",
                            op, left_ins, right
                        ),
                        line,
                    )
                }
            };
            sounds.push(Sound::new(PSound::new(right.get_note(), o, d), true));
            Chord::new(sounds)
        }
        _ => {
            return new_error(
                format!(
                    "unknown operator: op: '{}'  left: {:?}  right: {:?}",
                    op, left, right
                ),
                line,
            )
        }
    };
    let obj: Box<dyn Object> = Box::new(chord);
    obj
}

fn eval_chord_infix_expr(op: &str, left: Chord, right: Chord, line: usize) -> Box<dyn Object> {
    let chord = match op {
        "+" => {
            let mut sounds = left.get_sounds();
            sounds.append(&mut right.get_sounds());
            sounds
        }
        _ => {
            return new_error(
                format!(
                    "unknown operator: op: '{}'  left: {:?}  right: {:?}",
                    op, left, right
                ),
                line,
            )
        }
    };
    let obj: Box<dyn Object> = Box::new(Chord::new(chord));
    obj
}

fn eval_sound_infix_expr(op: &str, left: Sound, right: Sound, line: usize) -> Box<dyn Object> {
    let chord = match op {
        "+" => vec![left, right],
        _ => {
            return new_error(
                format!(
                    "unknown operator: op: '{}'  left: {:?}  right: {:?}",
                    op, left, right
                ),
                line,
            )
        }
    };
    let obj: Box<dyn Object> = Box::new(Chord::new(chord));
    obj
}

fn eval_int_infix_expr(op: &str, left: i32, right: i32, line: usize) -> Box<dyn Object> {
    let int = match op {
        "+" => left + right,
        "-" => left - right,
        "/" => left / right,
        "*" => left * right,
        _ => {
            return new_error(
                format!(
                    "unknown operator: op: '{}'  left: '{}'  right: '{}'",
                    op, left, right
                ),
                line,
            )
        }
    };
    let obj: Box<dyn Object> = Box::new(IntObj { value: int });
    obj
}

fn eval_exprs(expr: Vec<Box<dyn Expression>>, env: &mut Env) -> Vec<Box<dyn Object>> {
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
                Identifier {
                    token: Token {
                        ttype: TokenType::Ident,
                        literal: note.to_string(),
                        line: ident.token.line,
                    },
                    value: note.to_string(),
                },
                env,
            );

            let ins = n_eval.inspect();
            match n_eval.get_type() {
                Type::Note(note) => note,
                _ => return new_error(ins, ident.token.line),
            }
        }
        _ => return new_error("missing note argument".to_string(), ident.token.line),
    };

    let oct_ = match n.get_note() {
        PNote::Space => Some("4"),
        _ => spl.next(),
    };

    let oct = match oct_ {
        Some(o) => {
            let oct_eval = eval_ident(
                Identifier {
                    token: Token {
                        ttype: TokenType::Ident,
                        literal: format!("o{}", o),
                        line: ident.token.line,
                    },
                    value: format!("o{}", o),
                },
                env,
            );

            match oct_eval.get_type() {
                Type::Octave(o) => o,
                _ => return new_error("invalid note arg 2 octave".to_string(), ident.token.line),
            }
        }
        _ => return Box::new(n),
    };

    let dur = match spl.next() {
        Some(d) => {
            let dur_eval = eval_ident(
                Identifier {
                    token: Token {
                        ttype: TokenType::Ident,
                        literal: format!("d{}", d),
                        line: ident.token.line,
                    },
                    value: format!("d{}", d),
                },
                env,
            );
            let ins = dur_eval.inspect();
            match dur_eval.get_type() {
                Type::Duration(d) => d,
                _ => {
                    return new_error(
                        format!("invalid note duration `{:?}`", ins),
                        ident.token.line,
                    )
                }
            }
        }
        _ => return new_error("missing note duration".to_string(), ident.token.line),
    };

    Box::new(Sound {
        sound: PSound {
            note: n.get_note(),
            octave: oct.get_oct(),
            duration: dur.get_dur(),
            effects: None,
        },
        modified: false,
    })
}

fn eval_ident(ident: Identifier, env: &Env) -> Box<dyn Object> {
    if let Some(val) = env.get(ident.get_value().as_str()) {
        return val.clone_obj();
    }

    if let Some(builtin) = BUILTINS.get(ident.get_value().as_str()) {
        return builtin.clone_obj();
    }

    if ident.get_value().contains('_') {
        return eval_note_ident(ident, env);
    }

    new_error(
        format!("not found: `{}`", ident.to_string()),
        ident.token.line,
    )
}

#[cfg(test)]
use crate::interpreter::lexer::Lexer;
#[cfg(test)]
use crate::interpreter::parser::Parser;

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
            let mut env = Env::new();
            let evaluated = eval(exp.to_node(), &mut env);
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
        let mut env = Env::new();
        let _ = eval(exp.to_node(), &mut env);
    }
}

#[test]
fn test_track() {
    let expr = "let a = track(a_3_8*);";
    let lex = Lexer::new(expr);
    let mut p = Parser::new(lex);
    let program = p.parse_program();
    let mut env = Env::new();

    assert_eq!(1, program.exprs.len());

    for exp in program.exprs {
        let obj = eval(exp.to_node(), &mut env);
        if let Type::Null = obj.get_type() {
            return;
        }
        panic!("expected Null object");
    }

    if let Some(_) = env.get("a") {
        return;
    }
    panic!("expected `a` identifier to be stored in the env");
}

#[test]
fn test_daniel_expr() {
    let expr = "let t = 1; 1 - t * 4 + 3;";
    let res = 0;
    let lex = Lexer::new(expr);
    let mut p = Parser::new(lex);
    let program = p.parse_program();

    assert_eq!(2, program.exprs.len());

    let mut env = Env::new();

    let exp = program.exprs.first().unwrap();
    let evaluated = eval(exp.to_node(), &mut env);
    let t = evaluated.get_type();
    match &t {
        Type::Null => {}
        _ => panic!("expected Int, got {:?}", t),
    }

    let exp = program.exprs.get(1).unwrap();
    let evaluated = eval(exp.to_node(), &mut env);
    let t = evaluated.get_type();
    match &t {
        Type::Int(i) => assert_eq!(&res, i),
        _ => panic!("expected Int, got {:?}", t),
    }
}
