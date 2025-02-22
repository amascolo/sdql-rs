#![cfg(test)]

// let src = "3";
// let (tokens, errs) = lexer().parse(src).into_output_errors();
//
// let tokens = tokens.unwrap();
// for (t, _span) in &tokens {
//     println!("{t}");
// }
// assert!(errs.is_empty());
//
// let tokens = tokens
//     .as_slice()
//     .map((src.len()..src.len()).into(), |(t, s)| (t, s));
// let (ast, parse_errs) = expr_parser()
//     .map_with(|ast, e| (ast, e.span()))
//     .parse(tokens)
//     .into_output_errors();
//
// dbg!(&parse_errs);
//
// let ((expr, _), _) = ast.unwrap();
// // let main = ast.get("main").unwrap();
// // let Func{args: _, span: _, body} = main;
//
// println!("{expr:?}")

use super::*;
use lexer::lexer;

fn check_expr(src: &str, exp: Expr) {
    let (tokens, _errs) = lexer().parse(src).into_output_errors();

    let tokens = tokens.unwrap();
    // for (t, _span) in &tokens {
    //     println!("{t}");
    // }
    // assert!(_errs.is_empty());

    let tokens = tokens
        .as_slice()
        .map((src.len()..src.len()).into(), |(t, s)| (t, s));
    let (ast, parse_errs) = expr_parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens)
        .into_output_errors();

    if !parse_errs.is_empty() {
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
        Expr::Not(Box::new((Expr::Value(Value::Bool(true)), (1..5).into()))),
    );
    check_expr("52", Expr::Value(Value::Num(52f64)));
    check_expr(
        "-52",
        Expr::Neg(Box::new((Expr::Value(Value::Num(52.0)), (1..3).into()))),
    );
    check_expr("52.1", Expr::Value(Value::Num(52.1f64)));
    check_expr("\"foo\"", Expr::Value(Value::Str("foo")));
}

// FIXME parse errors
#[test]
fn if_then_else() {
    check_expr(
        "if true then 0 else 1",
        Expr::If(
            Box::new((Expr::Value(Value::Bool(true)), (3..7).into())),
            Box::new((Expr::Value(Value::Num(0f64)), (13..14).into())),
            Box::new((Expr::Value(Value::Num(1f64)), (20..21).into())),
        ),
    );

    check_expr(
        "if (true) then (0) else (1)",
        Expr::If(
            Box::new((Expr::Value(Value::Bool(true)), (4..8).into())),
            Box::new((Expr::Value(Value::Num(0f64)), (16..17).into())),
            Box::new((Expr::Value(Value::Num(1f64)), (25..26).into())),
        ),
    );

    check_expr(
        "if (!true) then (0) else (1)",
        Expr::If(
            Box::new((
                Expr::Not(Box::new((Expr::Value(Value::Bool(true)), (5..9).into()))),
                (4..9).into(),
            )),
            Box::new((Expr::Value(Value::Num(0f64)), (17..18).into())),
            Box::new((Expr::Value(Value::Num(1f64)), (26..27).into())),
        ),
    );
}

#[test]
fn let_bindings() {
    check_expr(
        "let x = 1 in 2",
        Expr::Let(
            "x",
            Box::new((Expr::Value(Value::Num(1.0)), (8..9).into())),
            Box::new((Expr::Value(Value::Num(2.0)), (13..14).into())),
        ),
    );

    check_expr(
        "let    x  =    (1) in    2",
        Expr::Let(
            "x",
            Box::new((Expr::Value(Value::Num(1.0)), (16..17).into())),
            Box::new((Expr::Value(Value::Num(2.0)), (25..26).into())),
        ),
    );

    check_expr(
        "let x_1 = 1 in 2",
        Expr::Let(
            "x_1",
            Box::new((Expr::Value(Value::Num(1.0)), (10..11).into())),
            Box::new((Expr::Value(Value::Num(2.0)), (15..16).into())),
        ),
    );

    check_expr(
        "let X = 1 in
            2",
        Expr::Let(
            "X",
            Box::new((Expr::Value(Value::Num(1.0)), (8..9).into())),
            Box::new((Expr::Value(Value::Num(2.0)), (25..26).into())),
        ),
    );
}

#[test]
fn arithmetic() {
    check_expr(
        "2 * 3",
        Expr::Binary(
            Box::new((Expr::Value(Value::Num(2f64)), (0..1).into())),
            BinaryOp::Mul,
            Box::new((Expr::Value(Value::Num(3f64)), (4..5).into())),
        ),
    );

    check_expr(
        "2 + 3",
        Expr::Binary(
            Box::new((Expr::Value(Value::Num(2f64)), (0..1).into())),
            BinaryOp::Add,
            Box::new((Expr::Value(Value::Num(3f64)), (4..5).into())),
        ),
    );

    check_expr(
        "2 / 3",
        Expr::Binary(
            Box::new((Expr::Value(Value::Num(2f64)), (0..1).into())),
            BinaryOp::Div,
            Box::new((Expr::Value(Value::Num(3f64)), (4..5).into())),
        ),
    );

    check_expr(
        "2 - 3",
        Expr::Binary(
            Box::new((Expr::Value(Value::Num(2f64)), (0..1).into())),
            BinaryOp::Sub,
            Box::new((Expr::Value(Value::Num(3f64)), (4..5).into())),
        ),
    );

    check_expr(
        "2 + 1 * 3",
        Expr::Binary(
            Box::new((Expr::Value(Value::Num(2f64)), (0..1).into())),
            BinaryOp::Add,
            Box::new((
                Expr::Binary(
                    Box::new((Expr::Value(Value::Num(1f64)), (4..5).into())),
                    BinaryOp::Mul,
                    Box::new((Expr::Value(Value::Num(3f64)), (8..9).into())),
                ),
                (4..9).into(),
            )),
        ),
    );

    check_expr(
        "2 * 1 + 3",
        Expr::Binary(
            Box::new((
                Expr::Binary(
                    Box::new((Expr::Value(Value::Num(2f64)), (0..1).into())),
                    BinaryOp::Mul,
                    Box::new((Expr::Value(Value::Num(1f64)), (4..5).into())),
                ),
                (0..5).into(),
            )),
            BinaryOp::Add,
            Box::new((Expr::Value(Value::Num(3f64)), (8..9).into())),
        ),
    );

    check_expr(
        "6 / 3 * 2",
        Expr::Binary(
            Box::new((
                Expr::Binary(
                    Box::new((Expr::Value(Value::Num(6f64)), (0..1).into())),
                    BinaryOp::Div,
                    Box::new((Expr::Value(Value::Num(3f64)), (4..5).into())),
                ),
                (0..5).into(),
            )),
            BinaryOp::Mul,
            Box::new((Expr::Value(Value::Num(2f64)), (8..9).into())),
        ),
    );

    check_expr(
        "2 < 3",
        Expr::Binary(
            Box::new((Expr::Value(Value::Num(2f64)), (0..1).into())),
            BinaryOp::Less,
            Box::new((Expr::Value(Value::Num(3f64)), (4..5).into())),
        ),
    );

    check_expr(
        "2 < 3 * 1",
        Expr::Binary(
            Box::new((Expr::Value(Value::Num(2f64)), (0..1).into())),
            BinaryOp::Less,
            Box::new((
                Expr::Binary(
                    Box::new((Expr::Value(Value::Num(3f64)), (4..5).into())),
                    BinaryOp::Mul,
                    Box::new((Expr::Value(Value::Num(1f64)), (8..9).into())),
                ),
                (4..9).into(),
            )),
        ),
    );

    check_expr(
        "2 < (3 * 1)",
        Expr::Binary(
            Box::new((Expr::Value(Value::Num(2f64)), (0..1).into())),
            BinaryOp::Less,
            Box::new((
                Expr::Binary(
                    Box::new((Expr::Value(Value::Num(3f64)), (5..6).into())),
                    BinaryOp::Mul,
                    Box::new((Expr::Value(Value::Num(1f64)), (9..10).into())),
                ),
                (5..10).into(),
            )),
        ),
    );

    check_expr(
        "true && false",
        Expr::Binary(
            Box::new((Expr::Value(Value::Bool(true)), (0..4).into())),
            BinaryOp::And,
            Box::new((Expr::Value(Value::Bool(false)), (8..13).into())),
        ),
    );

    check_expr(
        "true || false",
        Expr::Binary(
            Box::new((Expr::Value(Value::Bool(true)), (0..4).into())),
            BinaryOp::Or,
            Box::new((Expr::Value(Value::Bool(false)), (8..13).into())),
        ),
    );

    check_expr(
        "true && false || true",
        Expr::Binary(
            Box::new((
                Expr::Binary(
                    Box::new((Expr::Value(Value::Bool(true)), (0..4).into())),
                    BinaryOp::And,
                    Box::new((Expr::Value(Value::Bool(false)), (8..13).into())),
                ),
                (0..13).into(),
            )),
            BinaryOp::Or,
            Box::new((Expr::Value(Value::Bool(true)), (17..21).into())),
        ),
    );

    check_expr(
        "true || false && true",
        Expr::Binary(
            Box::new((Expr::Value(Value::Bool(true)), (0..4).into())),
            BinaryOp::Or,
            Box::new((
                Expr::Binary(
                    Box::new((Expr::Value(Value::Bool(false)), (8..13).into())),
                    BinaryOp::And,
                    Box::new((Expr::Value(Value::Bool(true)), (17..21).into())),
                ),
                (8..21).into(),
            )),
        ),
    );
}

#[test]
fn comments() {
    check_expr(
        "let x = y in z // comment for let",
        Expr::Let(
            "x",
            Box::new((Expr::Local("y"), (8..9).into())),
            Box::new((Expr::Local("z"), (13..14).into())),
        ),
    )
}

#[test]
fn dicts() {
    check_expr(
        "{k -> v}",
        Expr::Dict(vec![Pair {
            key: (Expr::Local("k"), (1..2).into()),
            value: (Expr::Local("v"), (6..7).into()),
        }]),
    )
}

#[test]
fn records() {
    check_expr(
        "<a = 1, b = 2>",
        Expr::Record(vec![
            Pair {
                key: (Expr::Local("a"), (1..2).into()),
                value: (Expr::Value(Value::Num(1f64)), (5..6).into()),
            },
            Pair {
                key: (Expr::Local("b"), (8..9).into()),
                value: (Expr::Value(Value::Num(2f64)), (12..13).into()),
            },
        ]),
    )
}

#[test]
fn sum() {
    check_expr(
        "sum(<k,v> <- X) v",
        Expr::Sum(Box::new(Sum {
            key: (Expr::Local("k"), (5..6).into()),
            value: (Expr::Local("v"), (7..8).into()),
            head: (Expr::Local("X"), (13..14).into()),
            body: (Expr::Local("v"), (16..17).into()),
        })),
    );

    check_expr(
        "sum(<k,v> <- X) {k -> v}",
        Expr::Sum(Box::new(Sum {
            key: (Expr::Local("k"), (5..6).into()),
            value: (Expr::Local("v"), (7..8).into()),
            head: (Expr::Local("X"), (13..14).into()),
            body: (
                Expr::Dict(vec![Pair {
                    key: (Expr::Local("k"), (17..18).into()),
                    value: (Expr::Local("v"), (22..23).into()),
                }]),
                (16..24).into(),
            ),
        })),
    );
}
