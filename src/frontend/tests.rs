#![cfg(test)]

use super::*;
use crate::ir::expr::{BinaryOp, DictEntry, Expr};
use crate::ir::r#type::DictHint;
use lexer::lexer;

fn check_expr(src: &str, exp: Expr) {
    let (tokens, _errs) = lexer().parse(src).into_output_errors();

    let tokens = tokens.unwrap();
    let tokens_for_debug = tokens.clone();

    let tokens = tokens
        .as_slice()
        .map((src.len()..src.len()).into(), |Spanned(t, s)| (t, s));
    let (ast, parse_errs) = expr_parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens)
        .into_output_errors();

    if !parse_errs.is_empty() {
        for Spanned(t, _span) in &tokens_for_debug {
            println!("{t}");
        }
        assert!(_errs.is_empty());
        dbg!(&parse_errs);
    }

    let (Spanned(expr, _), _) = ast.unwrap();
    assert_eq!(expr, exp);
}

#[test]
fn constants() {
    check_expr("true", Expr::Bool { val: true });
    check_expr("false", Expr::Bool { val: false });
    check_expr(
        "!true",
        Expr::Unary {
            op: UnaryOp::Not,
            expr: Spanned(Box::new(Expr::Bool { val: true }), (1..5).into()),
        },
    );
    check_expr("52", Expr::Int { val: 52 });
    check_expr("@long 52", Expr::Long { val: 52 });
    check_expr(
        "-52",
        Expr::Unary {
            op: UnaryOp::Neg,
            expr: Spanned(Box::new(Expr::Int { val: 52 }), (1..3).into()),
        },
    );
    check_expr("52.1", Expr::Real { val: 52.1f64 });
    check_expr(
        "\"foo\"",
        Expr::String {
            val: "foo",
            max_len: None,
        },
    );
    check_expr(
        "@varchar(3) \"foo\"",
        Expr::String {
            val: "foo",
            max_len: Some(3),
        },
    );
    check_expr(
        "date(20250525)",
        Expr::Date {
            val: crate::date!(20250525),
        },
    );
}

#[test]
fn if_then_else() {
    check_expr(
        "if true then 0 else 1",
        Expr::If {
            r#if: Spanned(Box::new(Expr::Bool { val: true }), (3..7).into()),
            then: Spanned(Box::new(Expr::Int { val: 0 }), (13..14).into()),
            r#else: Some(Spanned(Box::new(Expr::Int { val: 1 }), (20..21).into())),
        },
    );

    check_expr(
        "if (true) then (0) else (1)",
        Expr::If {
            r#if: Spanned(Box::new(Expr::Bool { val: true }), (4..8).into()),
            then: Spanned(Box::new(Expr::Int { val: 0 }), (16..17).into()),
            r#else: Some(Spanned(Box::new(Expr::Int { val: 1 }), (25..26).into())),
        },
    );

    check_expr(
        "if (!true) then (0) else (1)",
        Expr::If {
            r#if: Spanned(
                Box::new(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Spanned(Box::new(Expr::Bool { val: true }), (5..9).into()),
                }),
                (4..9).into(),
            ),
            then: Spanned(Box::new(Expr::Int { val: 0 }), (17..18).into()),
            r#else: Some(Spanned(Box::new(Expr::Int { val: 1 }), (26..27).into())),
        },
    );
}

#[test]
fn let_bindings() {
    check_expr(
        "let x = 1 in 2",
        Expr::Let {
            lhs: "x",
            rhs: Spanned(Box::new(Expr::Int { val: 1 }), (8..9).into()),
            cont: Spanned(Box::new(Expr::Int { val: 2 }), (13..14).into()),
        },
    );

    check_expr(
        "let    x  =    (1) in    2",
        Expr::Let {
            lhs: "x",
            rhs: Spanned(Box::new(Expr::Int { val: 1 }), (16..17).into()),
            cont: Spanned(Box::new(Expr::Int { val: 2 }), (25..26).into()),
        },
    );

    check_expr(
        "let x_1 = 1 in 2",
        Expr::Let {
            lhs: "x_1",
            rhs: Spanned(Box::new(Expr::Int { val: 1 }), (10..11).into()),
            cont: Spanned(Box::new(Expr::Int { val: 2 }), (15..16).into()),
        },
    );

    check_expr(
        "let X = 1 in
            2",
        Expr::Let {
            lhs: "X",
            rhs: Spanned(Box::new(Expr::Int { val: 1 }), (8..9).into()),
            cont: Spanned(Box::new(Expr::Int { val: 2 }), (25..26).into()),
        },
    );
}

#[test]
fn arithmetic() {
    check_expr(
        "2 * 3",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Int { val: 2 }), (0..1).into()),
            op: BinaryOp::Mul,
            rhs: Spanned(Box::new(Expr::Int { val: 3 }), (4..5).into()),
        },
    );

    check_expr(
        "2 + 3",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Int { val: 2 }), (0..1).into()),
            op: BinaryOp::Add,
            rhs: Spanned(Box::new(Expr::Int { val: 3 }), (4..5).into()),
        },
    );

    check_expr(
        "2 / 3",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Int { val: 2 }), (0..1).into()),
            op: BinaryOp::Div,
            rhs: Spanned(Box::new(Expr::Int { val: 3 }), (4..5).into()),
        },
    );

    check_expr(
        "2 - 3",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Int { val: 2 }), (0..1).into()),
            op: BinaryOp::Sub,
            rhs: Spanned(Box::new(Expr::Int { val: 3 }), (4..5).into()),
        },
    );

    check_expr(
        "2 + 1 * 3",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Int { val: 2 }), (0..1).into()),
            op: BinaryOp::Add,
            rhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(Expr::Int { val: 1 }), (4..5).into()),
                    op: BinaryOp::Mul,
                    rhs: Spanned(Box::new(Expr::Int { val: 3 }), (8..9).into()),
                }),
                (4..9).into(),
            ),
        },
    );

    check_expr(
        "2 * 1 + 3",
        Expr::Binary {
            lhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(Expr::Int { val: 2 }), (0..1).into()),
                    op: BinaryOp::Mul,
                    rhs: Spanned(Box::new(Expr::Int { val: 1 }), (4..5).into()),
                }),
                (0..5).into(),
            ),
            op: BinaryOp::Add,
            rhs: Spanned(Box::new(Expr::Int { val: 3 }), (8..9).into()),
        },
    );

    check_expr(
        "6 / 3 * 2",
        Expr::Binary {
            lhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(Expr::Int { val: 6 }), (0..1).into()),
                    op: BinaryOp::Div,
                    rhs: Spanned(Box::new(Expr::Int { val: 3 }), (4..5).into()),
                }),
                (0..5).into(),
            ),
            op: BinaryOp::Mul,
            rhs: Spanned(Box::new(Expr::Int { val: 2 }), (8..9).into()),
        },
    );

    check_expr(
        "2 < 3",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Int { val: 2 }), (0..1).into()),
            op: BinaryOp::Less,
            rhs: Spanned(Box::new(Expr::Int { val: 3 }), (4..5).into()),
        },
    );

    check_expr(
        "2 < 3 * 1",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Int { val: 2 }), (0..1).into()),
            op: BinaryOp::Less,
            rhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(Expr::Int { val: 3 }), (4..5).into()),
                    op: BinaryOp::Mul,
                    rhs: Spanned(Box::new(Expr::Int { val: 1 }), (8..9).into()),
                }),
                (4..9).into(),
            ),
        },
    );

    check_expr(
        "2 < (3 * 1)",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Int { val: 2 }), (0..1).into()),
            op: BinaryOp::Less,
            rhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(Expr::Int { val: 3 }), (5..6).into()),
                    op: BinaryOp::Mul,
                    rhs: Spanned(Box::new(Expr::Int { val: 1 }), (9..10).into()),
                }),
                (5..10).into(),
            ),
        },
    );

    check_expr(
        "true && false",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Bool { val: true }), (0..4).into()),
            op: BinaryOp::And,
            rhs: Spanned(Box::new(Expr::Bool { val: false }), (8..13).into()),
        },
    );

    check_expr(
        "true || false",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Bool { val: true }), (0..4).into()),
            op: BinaryOp::Or,
            rhs: Spanned(Box::new(Expr::Bool { val: false }), (8..13).into()),
        },
    );

    check_expr(
        "true && false || true",
        Expr::Binary {
            lhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(Expr::Bool { val: true }), (0..4).into()),
                    op: BinaryOp::And,
                    rhs: Spanned(Box::new(Expr::Bool { val: false }), (8..13).into()),
                }),
                (0..13).into(),
            ),
            op: BinaryOp::Or,
            rhs: Spanned(Box::new(Expr::Bool { val: true }), (17..21).into()),
        },
    );

    check_expr(
        "true || false && true",
        Expr::Binary {
            lhs: Spanned(Box::new(Expr::Bool { val: true }), (0..4).into()),
            op: BinaryOp::Or,
            rhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(Expr::Bool { val: false }), (8..13).into()),
                    op: BinaryOp::And,
                    rhs: Spanned(Box::new(Expr::Bool { val: true }), (17..21).into()),
                }),
                (8..21).into(),
            ),
        },
    );
}

#[test]
fn comments() {
    check_expr(
        "let x = y in z // comment for let",
        Expr::Let {
            lhs: "x",
            rhs: Spanned(Box::new(Expr::Sym { val: "y" }), (8..9).into()),
            cont: Spanned(Box::new(Expr::Sym { val: "z" }), (13..14).into()),
        },
    )
}

#[test]
fn dicts() {
    check_expr(
        "{k -> v}",
        Expr::Dict {
            map: vec![DictEntry {
                key: Spanned(Expr::Sym { val: "k" }, (1..2).into()),
                val: Spanned(Expr::Sym { val: "v" }, (6..7).into()),
            }],
            hint: None,
        },
    );

    check_expr(
        "@hashdict {k -> v}",
        Expr::Dict {
            map: vec![DictEntry {
                key: Spanned(Expr::Sym { val: "k" }, (11..12).into()),
                val: Spanned(Expr::Sym { val: "v" }, (16..17).into()),
            }],
            hint: Some(DictHint::HashDict),
        },
    );

    check_expr(
        "@sortdict {k -> v}",
        Expr::Dict {
            map: vec![DictEntry {
                key: Spanned(Expr::Sym { val: "k" }, (11..12).into()),
                val: Spanned(Expr::Sym { val: "v" }, (16..17).into()),
            }],
            hint: Some(DictHint::SortDict),
        },
    );

    check_expr(
        "@smallvecdict {k -> v}",
        Expr::Dict {
            map: vec![DictEntry {
                key: Spanned(Expr::Sym { val: "k" }, (15..16).into()),
                val: Spanned(Expr::Sym { val: "v" }, (20..21).into()),
            }],
            hint: Some(DictHint::SmallVecDict),
        },
    );

    check_expr(
        "@vec {k -> v}",
        Expr::Dict {
            map: vec![DictEntry {
                key: Spanned(Expr::Sym { val: "k" }, (6..7).into()),
                val: Spanned(Expr::Sym { val: "v" }, (11..12).into()),
            }],
            hint: Some(DictHint::Vec),
        },
    );
}

#[test]
fn sets() {
    check_expr("{}", Expr::Set(vec![]));

    check_expr(
        "{1}",
        Expr::Set(vec![Spanned(Expr::Int { val: 1 }, (1..2).into())]),
    );

    check_expr(
        "{x}",
        Expr::Set(vec![Spanned(Expr::Sym { val: "x" }, (1..2).into())]),
    );

    check_expr(
        "{0, 1}",
        Expr::Set(vec![
            Spanned(Expr::Int { val: 0 }, (1..2).into()),
            Spanned(Expr::Int { val: 1 }, (4..5).into()),
        ]),
    );

    check_expr(
        "{x, y}",
        Expr::Set(vec![
            Spanned(Expr::Sym { val: "x" }, (1..2).into()),
            Spanned(Expr::Sym { val: "y" }, (4..5).into()),
        ]),
    );
}

#[test]
fn records() {
    check_expr(
        "<a = 1, b = 2>",
        Expr::Record {
            vals: vec![
                RecordValue {
                    name: "a".into(),
                    val: Spanned(Expr::Int { val: 1 }, (5..6).into()),
                },
                RecordValue {
                    name: "b".into(),
                    val: Spanned(Expr::Int { val: 2 }, (12..13).into()),
                },
            ],
        },
    );
}

#[test]
fn fields() {
    check_expr(
        "x.foo",
        Expr::Field {
            expr: Spanned(Box::new(Expr::Sym { val: "x" }), (0..1).into()),
            field: "foo".into(),
        },
    );

    check_expr(
        "x.foo * y.doo",
        Expr::Binary {
            lhs: Spanned(
                Box::new(Expr::Field {
                    expr: Spanned(Box::new(Expr::Sym { val: "x" }), (0..1).into()),
                    field: "foo".into(),
                }),
                (0..5).into(),
            ),
            op: BinaryOp::Mul,
            rhs: Spanned(
                Box::new(Expr::Field {
                    expr: Spanned(Box::new(Expr::Sym { val: "y" }), (8..9).into()),
                    field: "doo".into(),
                }),
                (8..13).into(),
            ),
        },
    );

    check_expr(
        "x.foo.doo",
        Expr::Field {
            expr: Spanned(
                Box::new(Expr::Field {
                    expr: Spanned(Box::new(Expr::Sym { val: "x" }), (0..1).into()),
                    field: "foo".into(),
                }),
                (0..5).into(),
            ),
            field: "doo".into(),
        },
    );

    check_expr(
        "< first = 1 >.first",
        Expr::Field {
            expr: Spanned(
                Box::new(Expr::Record {
                    vals: vec![RecordValue {
                        name: "first".into(),
                        val: Spanned(Expr::Int { val: 1 }, (10..11).into()),
                    }],
                }),
                (0..13).into(),
            ),
            field: "first".into(),
        },
    );
}

#[test]
fn gets() {
    check_expr(
        "x(1)",
        Expr::Get {
            lhs: Spanned(Box::new(Expr::Sym { val: "x" }), (0..1).into()),
            rhs: Spanned(Box::new(Expr::Int { val: 1 }), (2..3).into()),
        },
    );

    check_expr(
        "x(y)",
        Expr::Get {
            lhs: Spanned(Box::new(Expr::Sym { val: "x" }), (0..1).into()),
            rhs: Spanned(Box::new(Expr::Sym { val: "y" }), (2..3).into()),
        },
    );

    check_expr(
        "x(y)(z)",
        Expr::Get {
            lhs: Spanned(
                Box::new(Expr::Get {
                    lhs: Spanned(Box::new(Expr::Sym { val: "x" }), (0..1).into()),
                    rhs: Spanned(Box::new(Expr::Sym { val: "y" }), (2..3).into()),
                }),
                (0..4).into(),
            ),
            rhs: Spanned(Box::new(Expr::Sym { val: "z" }), (5..6).into()),
        },
    );

    // FIXME
    // check_expr(
    //     "< first = 1 >(1)",
    //     Expr::Get {
    //         lhs: Spanned(
    //             Box::new(Expr::Record {
    //                 vals: vec![RecordValue {
    //                     name: "first".into(),
    //                     val: Spanned(Expr::Int { val: 1 }, (10..11).into()),
    //                 }],
    //             }),
    //             (0..13).into(),
    //         ),
    //         rhs: Spanned(Box::new(Expr::Int { val: 1 }), (16..17).into()),
    //     },
    // );

    // TODO get on dict literal

    // TODO get on set literal
}

#[test]
fn sum() {
    check_expr(
        "sum(<k,v> <- X) v",
        Expr::Sum {
            key: &"k",
            val: &"v",
            head: Spanned(Box::new(Expr::Sym { val: "X" }), (13..14).into()),
            body: Spanned(Box::new(Expr::Sym { val: "v" }), (16..17).into()),
        },
    );

    check_expr(
        "sum(<k,v> <- X) {k -> v}",
        Expr::Sum {
            key: &"k",
            val: &"v",
            head: Spanned(Box::new(Expr::Sym { val: "X" }), (13..14).into()),
            body: Spanned(
                Box::new(Expr::Dict {
                    map: vec![DictEntry {
                        key: Spanned(Expr::Sym { val: "k" }, (17..18).into()),
                        val: Spanned(Expr::Sym { val: "v" }, (22..23).into()),
                    }],
                    hint: None,
                }),
                (16..24).into(),
            ),
        },
    );
}

#[test]
fn load() {
    check_expr(
        "load[<foobar: @vec {int -> real}>](\"foo.csv\")",
        Expr::Load {
            r#type: Type::Record(vec![RecordType {
                name: "foobar".into(),
                r#type: Type::Dict {
                    key: Box::new(Type::Int),
                    val: Box::new(Type::Real),
                    hint: Some(DictHint::Vec),
                },
            }]),
            path: "foo.csv",
        },
    );
}

#[test]
fn concat() {
    check_expr(
        "concat(k,v)",
        Expr::Concat {
            lhs: Spanned(Expr::Sym { val: "k" }, (7..8).into()).boxed(),
            rhs: Spanned(Expr::Sym { val: "v" }, (9..10).into()).boxed(),
        },
    )
}

// #[test]
// fn external() {
//     todo!()
// }

#[test]
fn promote() {
    check_expr(
        "promote[real](1)",
        Expr::Promote {
            promo: Type::Real,
            expr: Spanned(Expr::Int { val: 1 }, (14..15).into()).boxed(),
        },
    )
}

#[test]
fn unique() {
    check_expr(
        "unique(x)",
        Expr::Unique {
            expr: Spanned(Box::new(Expr::Sym { val: "x" }), (7..8).into()),
        },
    );
}

#[test]
fn sum_nested_if() {
    no_check_expr(
        "sum(<i,_> <- range(lineitem.size))
           if(true) then
               i
           else
               i",
    )
}

fn no_check_expr(src: &str) {
    let (tokens, _errs) = lexer().parse(src).into_output_errors();

    let tokens = tokens.unwrap();
    let tokens_for_debug = tokens.clone();

    let tokens = tokens
        .as_slice()
        .map((src.len()..src.len()).into(), |Spanned(t, s)| (t, s));
    let (ast, parse_errs) = expr_parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens)
        .into_output_errors();

    if !parse_errs.is_empty() {
        for Spanned(t, _span) in &tokens_for_debug {
            println!("{t}");
        }
        dbg!(&_errs);
        assert!(_errs.is_empty());
        dbg!(&parse_errs);
    }

    let (Spanned(expr, _), _) = ast.unwrap();
    println!("{expr}");
}

#[test]
fn tpch_q3() {
    let prog = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q3.sdql"));
    no_check_expr(prog);
}

#[test]
fn tpch_q6() {
    let prog = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q6.sdql"));
    no_check_expr(prog);
}
