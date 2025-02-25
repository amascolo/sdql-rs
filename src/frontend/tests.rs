#![cfg(test)]

use super::*;
use expr::{BinaryOp, DictEntry, Expr, Value};
use lexer::lexer;
use r#type::Type;

fn check_expr(src: &str, exp: Expr) {
    let (tokens, _errs) = lexer().parse(src).into_output_errors();

    let tokens = tokens.unwrap();
    let tokens_for_debug = tokens.clone();

    let tokens = tokens
        .as_slice()
        .map((src.len()..src.len()).into(), |(t, s)| (t, s));
    let (ast, parse_errs) = expr_parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens)
        .into_output_errors();

    if !parse_errs.is_empty() {
        for (t, _span) in &tokens_for_debug {
            println!("{t}");
        }
        assert!(_errs.is_empty());
        dbg!(&parse_errs);
    }

    let ((expr, _), _) = ast.unwrap();
    assert_eq!(expr, exp);
}

#[test]
fn constants() {
    check_expr("true", Expr::Value(Value::Bool(true)));
    check_expr("false", Expr::Value(Value::Bool(false)));
    check_expr(
        "!true",
        Expr::Unary {
            op: UnaryOp::Not,
            expr: Box::new((Expr::Value(Value::Bool(true)), (1..5).into())),
        },
    );
    check_expr("52", Expr::Value(Value::Float(52f64)));
    check_expr(
        "-52",
        Expr::Unary {
            op: UnaryOp::Neg,
            expr: Box::new((Expr::Value(Value::Float(52.0)), (1..3).into())),
        },
    );
    check_expr("52.1", Expr::Value(Value::Float(52.1f64)));
    check_expr("\"foo\"", Expr::Value(Value::String("foo")));
}

#[test]
fn if_then_else() {
    check_expr(
        "if true then 0 else 1",
        Expr::If {
            r#if: Box::new((Expr::Value(Value::Bool(true)), (3..7).into())),
            then: Box::new((Expr::Value(Value::Float(0f64)), (13..14).into())),
            r#else: Some(Box::new((Expr::Value(Value::Float(1f64)), (20..21).into()))),
        },
    );

    check_expr(
        "if (true) then (0) else (1)",
        Expr::If {
            r#if: Box::new((Expr::Value(Value::Bool(true)), (4..8).into())),
            then: Box::new((Expr::Value(Value::Float(0f64)), (16..17).into())),
            r#else: Some(Box::new((Expr::Value(Value::Float(1f64)), (25..26).into()))),
        },
    );

    check_expr(
        "if (!true) then (0) else (1)",
        Expr::If {
            r#if: Box::new((
                Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new((Expr::Value(Value::Bool(true)), (5..9).into())),
                },
                (4..9).into(),
            )),
            then: Box::new((Expr::Value(Value::Float(0f64)), (17..18).into())),
            r#else: Some(Box::new((Expr::Value(Value::Float(1f64)), (26..27).into()))),
        },
    );
}

#[test]
fn let_bindings() {
    check_expr(
        "let x = 1 in 2",
        Expr::Let {
            lhs: "x",
            rhs: Box::new((Expr::Value(Value::Float(1.0)), (8..9).into())),
            cont: Box::new((Expr::Value(Value::Float(2.0)), (13..14).into())),
        },
    );

    check_expr(
        "let    x  =    (1) in    2",
        Expr::Let {
            lhs: "x",
            rhs: Box::new((Expr::Value(Value::Float(1.0)), (16..17).into())),
            cont: Box::new((Expr::Value(Value::Float(2.0)), (25..26).into())),
        },
    );

    check_expr(
        "let x_1 = 1 in 2",
        Expr::Let {
            lhs: "x_1",
            rhs: Box::new((Expr::Value(Value::Float(1.0)), (10..11).into())),
            cont: Box::new((Expr::Value(Value::Float(2.0)), (15..16).into())),
        },
    );

    check_expr(
        "let X = 1 in
            2",
        Expr::Let {
            lhs: "X",
            rhs: Box::new((Expr::Value(Value::Float(1.0)), (8..9).into())),
            cont: Box::new((Expr::Value(Value::Float(2.0)), (25..26).into())),
        },
    );
}

#[test]
fn arithmetic() {
    check_expr(
        "2 * 3",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Float(2f64)), (0..1).into())),
            op: BinaryOp::Mul,
            rhs: Box::new((Expr::Value(Value::Float(3f64)), (4..5).into())),
        },
    );

    check_expr(
        "2 + 3",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Float(2f64)), (0..1).into())),
            op: BinaryOp::Add,
            rhs: Box::new((Expr::Value(Value::Float(3f64)), (4..5).into())),
        },
    );

    check_expr(
        "2 / 3",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Float(2f64)), (0..1).into())),
            op: BinaryOp::Div,
            rhs: Box::new((Expr::Value(Value::Float(3f64)), (4..5).into())),
        },
    );

    check_expr(
        "2 - 3",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Float(2f64)), (0..1).into())),
            op: BinaryOp::Sub,
            rhs: Box::new((Expr::Value(Value::Float(3f64)), (4..5).into())),
        },
    );

    check_expr(
        "2 + 1 * 3",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Float(2f64)), (0..1).into())),
            op: BinaryOp::Add,
            rhs: Box::new((
                Expr::Binary {
                    lhs: Box::new((Expr::Value(Value::Float(1f64)), (4..5).into())),
                    op: BinaryOp::Mul,
                    rhs: Box::new((Expr::Value(Value::Float(3f64)), (8..9).into())),
                },
                (4..9).into(),
            )),
        },
    );

    check_expr(
        "2 * 1 + 3",
        Expr::Binary {
            lhs: Box::new((
                Expr::Binary {
                    lhs: Box::new((Expr::Value(Value::Float(2f64)), (0..1).into())),
                    op: BinaryOp::Mul,
                    rhs: Box::new((Expr::Value(Value::Float(1f64)), (4..5).into())),
                },
                (0..5).into(),
            )),
            op: BinaryOp::Add,
            rhs: Box::new((Expr::Value(Value::Float(3f64)), (8..9).into())),
        },
    );

    check_expr(
        "6 / 3 * 2",
        Expr::Binary {
            lhs: Box::new((
                Expr::Binary {
                    lhs: Box::new((Expr::Value(Value::Float(6f64)), (0..1).into())),
                    op: BinaryOp::Div,
                    rhs: Box::new((Expr::Value(Value::Float(3f64)), (4..5).into())),
                },
                (0..5).into(),
            )),
            op: BinaryOp::Mul,
            rhs: Box::new((Expr::Value(Value::Float(2f64)), (8..9).into())),
        },
    );

    check_expr(
        "2 < 3",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Float(2f64)), (0..1).into())),
            op: BinaryOp::Less,
            rhs: Box::new((Expr::Value(Value::Float(3f64)), (4..5).into())),
        },
    );

    check_expr(
        "2 < 3 * 1",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Float(2f64)), (0..1).into())),
            op: BinaryOp::Less,
            rhs: Box::new((
                Expr::Binary {
                    lhs: Box::new((Expr::Value(Value::Float(3f64)), (4..5).into())),
                    op: BinaryOp::Mul,
                    rhs: Box::new((Expr::Value(Value::Float(1f64)), (8..9).into())),
                },
                (4..9).into(),
            )),
        },
    );

    check_expr(
        "2 < (3 * 1)",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Float(2f64)), (0..1).into())),
            op: BinaryOp::Less,
            rhs: Box::new((
                Expr::Binary {
                    lhs: Box::new((Expr::Value(Value::Float(3f64)), (5..6).into())),
                    op: BinaryOp::Mul,
                    rhs: Box::new((Expr::Value(Value::Float(1f64)), (9..10).into())),
                },
                (5..10).into(),
            )),
        },
    );

    check_expr(
        "true && false",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Bool(true)), (0..4).into())),
            op: BinaryOp::And,
            rhs: Box::new((Expr::Value(Value::Bool(false)), (8..13).into())),
        },
    );

    check_expr(
        "true || false",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Bool(true)), (0..4).into())),
            op: BinaryOp::Or,
            rhs: Box::new((Expr::Value(Value::Bool(false)), (8..13).into())),
        },
    );

    check_expr(
        "true && false || true",
        Expr::Binary {
            lhs: Box::new((
                Expr::Binary {
                    lhs: Box::new((Expr::Value(Value::Bool(true)), (0..4).into())),
                    op: BinaryOp::And,
                    rhs: Box::new((Expr::Value(Value::Bool(false)), (8..13).into())),
                },
                (0..13).into(),
            )),
            op: BinaryOp::Or,
            rhs: Box::new((Expr::Value(Value::Bool(true)), (17..21).into())),
        },
    );

    check_expr(
        "true || false && true",
        Expr::Binary {
            lhs: Box::new((Expr::Value(Value::Bool(true)), (0..4).into())),
            op: BinaryOp::Or,
            rhs: Box::new((
                Expr::Binary {
                    lhs: Box::new((Expr::Value(Value::Bool(false)), (8..13).into())),
                    op: BinaryOp::And,
                    rhs: Box::new((Expr::Value(Value::Bool(true)), (17..21).into())),
                },
                (8..21).into(),
            )),
        },
    );
}

#[test]
fn comments() {
    check_expr(
        "let x = y in z // comment for let",
        Expr::Let {
            lhs: "x",
            rhs: Box::new((Expr::Sym("y"), (8..9).into())),
            cont: Box::new((Expr::Sym("z"), (13..14).into())),
        },
    )
}

#[test]
fn dicts() {
    check_expr(
        "{k -> v}",
        Expr::Dict {
            map: vec![DictEntry {
                key: (Expr::Sym("k"), (1..2).into()),
                val: (Expr::Sym("v"), (6..7).into()),
            }],
            hint: None,
        },
    );

    check_expr(
        "@hashdict {k -> v}",
        Expr::Dict {
            map: vec![DictEntry {
                key: (Expr::Sym("k"), (11..12).into()),
                val: (Expr::Sym("v"), (16..17).into()),
            }],
            hint: Some(DictHint::HashDict),
        },
    );

    check_expr(
        "@sortdict {k -> v}",
        Expr::Dict {
            map: vec![DictEntry {
                key: (Expr::Sym("k"), (11..12).into()),
                val: (Expr::Sym("v"), (16..17).into()),
            }],
            hint: Some(DictHint::SortDict),
        },
    );

    check_expr(
        "@smallvecdict {k -> v}",
        Expr::Dict {
            map: vec![DictEntry {
                key: (Expr::Sym("k"), (15..16).into()),
                val: (Expr::Sym("v"), (20..21).into()),
            }],
            hint: Some(DictHint::SmallVecDict),
        },
    );

    check_expr(
        "@vec {k -> v}",
        Expr::Dict {
            map: vec![DictEntry {
                key: (Expr::Sym("k"), (6..7).into()),
                val: (Expr::Sym("v"), (11..12).into()),
            }],
            hint: Some(DictHint::Vec),
        },
    );
}

#[test]
fn records() {
    check_expr(
        "<a = 1, b = 2>",
        Expr::Record(vec![
            RecordValue {
                name: ("a".into(), (1..2).into()),
                val: (Expr::Value(Value::Float(1f64)), (5..6).into()),
            },
            RecordValue {
                name: ("b".into(), (8..9).into()),
                val: (Expr::Value(Value::Float(2f64)), (12..13).into()),
            },
        ]),
    );
}

#[test]
fn fields() {
    check_expr(
        "x.name",
        Expr::Field {
            expr: Box::new((Expr::Sym("x"), (0..1).into())),
            field: "name".into(),
        },
    );

    check_expr(
        "x.foo * y.doo",
        Expr::Binary {
            lhs: Box::new((
                Expr::Field {
                    expr: Box::new((Expr::Sym("x"), (0..1).into())),
                    field: "foo".into(),
                },
                (0..5).into(),
            )),
            op: BinaryOp::Mul,
            rhs: Box::new((
                Expr::Field {
                    expr: Box::new((Expr::Sym("y"), (8..9).into())),
                    field: "doo".into(),
                },
                (8..13).into(),
            )),
        },
    );
}

#[test]
fn sum() {
    check_expr(
        "sum(<k,v> <- X) v",
        Expr::Sum {
            key: Box::new((Expr::Sym("k"), (5..6).into())),
            val: Box::new((Expr::Sym("v"), (7..8).into())),
            head: Box::new((Expr::Sym("X"), (13..14).into())),
            body: Box::new((Expr::Sym("v"), (16..17).into())),
        },
    );

    check_expr(
        "sum(<k,v> <- X) {k -> v}",
        Expr::Sum {
            key: Box::new((Expr::Sym("k"), (5..6).into())),
            val: Box::new((Expr::Sym("v"), (7..8).into())),
            head: Box::new((Expr::Sym("X"), (13..14).into())),
            body: Box::new((
                Expr::Dict {
                    map: vec![DictEntry {
                        key: (Expr::Sym("k"), (17..18).into()),
                        val: (Expr::Sym("v"), (22..23).into()),
                    }],
                    hint: None,
                },
                (16..24).into(),
            )),
        },
    );
}

#[test]
fn load() {
    check_expr(
        "load[{string -> bool}](\"foo.csv\")",
        Expr::Load {
            r#type: Type::Dict {
                key: Box::new(Type::String { max_len: None }),
                val: Box::new(Type::Bool),
                hint: None,
            },
            path: "foo.csv",
        },
    );
}
