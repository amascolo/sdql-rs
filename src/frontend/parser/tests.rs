use super::*;
use crate::ir::expr::{BinOp, DictEntry, Expr};
use crate::ir::r#type::DictHint;
use crate::no_span;
use sdql_runtime::date;

#[test]
fn constants() {
    assert_eq!(no_span!("true"), Expr::Bool { val: true });
    assert_eq!(no_span!("false"), Expr::Bool { val: false });
    assert_eq!(
        no_span!("!true"),
        Expr::Unary {
            op: UnaryOp::Not,
            expr: Spanned(Box::new(no_span!("true")), (1..5).into()),
        },
    );
    assert_eq!(no_span!("52"), Expr::Int { val: 52 });
    assert_eq!(no_span!("@long 52"), Expr::Long { val: 52 });
    assert_eq!(
        no_span!("-52"),
        Expr::Unary {
            op: UnaryOp::Neg,
            expr: Spanned(Box::new(no_span!("52")), (1..3).into()),
        },
    );
    assert_eq!(no_span!("52.1"), Expr::Real { val: 52.1f64 });
    assert_eq!(
        no_span!("\"foo\""),
        Expr::String {
            val: "foo",
            max_len: None,
        },
    );
    assert_eq!(
        no_span!("@varchar(3) \"foo\""),
        Expr::String {
            val: "foo",
            max_len: Some(3),
        },
    );
    assert_eq!(
        no_span!("date(20250525)"),
        Expr::Date {
            val: date!(20250525),
        },
    );
    assert_eq!(no_span!("x"), Expr::Sym { val: "x" });
}

#[test]
fn if_then_else() {
    assert_eq!(
        no_span!("if true then 0 else 1"),
        Expr::If {
            r#if: Spanned(Box::new(no_span!("true")), (3..7).into()),
            then: Spanned(Box::new(no_span!("0")), (13..14).into()),
            r#else: Some(Spanned(Box::new(no_span!("1")), (20..21).into())),
        },
    );

    assert_eq!(
        no_span!("if (true) then (0) else (1)"),
        Expr::If {
            r#if: Spanned(Box::new(no_span!("true")), (4..8).into()),
            then: Spanned(Box::new(no_span!("0")), (16..17).into()),
            r#else: Some(Spanned(Box::new(no_span!("1")), (25..26).into())),
        },
    );

    assert_eq!(
        no_span!("if (!true) then (0) else (1)"),
        Expr::If {
            r#if: Spanned(
                Box::new(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Spanned(Box::new(no_span!("true")), (5..9).into()),
                }),
                (4..9).into(),
            ),
            then: Spanned(Box::new(no_span!("0")), (17..18).into()),
            r#else: Some(Spanned(Box::new(no_span!("1")), (26..27).into())),
        },
    );
}

#[test]
fn let_bindings() {
    assert_eq!(
        no_span!("let x = 1 in 2"),
        Expr::Let {
            lhs: "x",
            rhs: Spanned(Box::new(no_span!("1")), (8..9).into()),
            cont: Spanned(Box::new(no_span!("2")), (13..14).into()),
        },
    );

    assert_eq!(
        no_span!("let    x  =    (1) in    2"),
        Expr::Let {
            lhs: "x",
            rhs: Spanned(Box::new(no_span!("1")), (16..17).into()),
            cont: Spanned(Box::new(no_span!("2")), (25..26).into()),
        },
    );

    assert_eq!(
        no_span!("let x_1 = 1 in 2"),
        Expr::Let {
            lhs: "x_1",
            rhs: Spanned(Box::new(no_span!("1")), (10..11).into()),
            cont: Spanned(Box::new(no_span!("2")), (15..16).into()),
        },
    );

    assert_eq!(
        no_span!(
            "let X = 1 in
            2"
        ),
        Expr::Let {
            lhs: "X",
            rhs: Spanned(Box::new(no_span!("1")), (8..9).into()),
            cont: Spanned(Box::new(no_span!("2")), (25..26).into()),
        },
    );
}

#[test]
fn arithmetic() {
    assert_eq!(
        no_span!("2 * 3"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("2")), (0..1).into()),
            op: BinOp::Mul,
            rhs: Spanned(Box::new(no_span!("3")), (4..5).into()),
        },
    );

    assert_eq!(
        no_span!("2 + 3"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("2")), (0..1).into()),
            op: BinOp::Add,
            rhs: Spanned(Box::new(no_span!("3")), (4..5).into()),
        },
    );

    assert_eq!(
        no_span!("2 / 3"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("2")), (0..1).into()),
            op: BinOp::Div,
            rhs: Spanned(Box::new(no_span!("3")), (4..5).into()),
        },
    );

    assert_eq!(
        no_span!("2 - 3"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("2")), (0..1).into()),
            op: BinOp::Sub,
            rhs: Spanned(Box::new(no_span!("3")), (4..5).into()),
        },
    );

    assert_eq!(
        no_span!("2 + 1 * 3"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("2")), (0..1).into()),
            op: BinOp::Add,
            rhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(no_span!("1")), (4..5).into()),
                    op: BinOp::Mul,
                    rhs: Spanned(Box::new(no_span!("3")), (8..9).into()),
                }),
                (4..9).into(),
            ),
        },
    );

    assert_eq!(
        no_span!("2 * 1 + 3"),
        Expr::Binary {
            lhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(no_span!("2")), (0..1).into()),
                    op: BinOp::Mul,
                    rhs: Spanned(Box::new(no_span!("1")), (4..5).into()),
                }),
                (0..5).into(),
            ),
            op: BinOp::Add,
            rhs: Spanned(Box::new(no_span!("3")), (8..9).into()),
        },
    );

    assert_eq!(
        no_span!("6 / 3 * 2"),
        Expr::Binary {
            lhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(no_span!("6")), (0..1).into()),
                    op: BinOp::Div,
                    rhs: Spanned(Box::new(no_span!("3")), (4..5).into()),
                }),
                (0..5).into(),
            ),
            op: BinOp::Mul,
            rhs: Spanned(Box::new(no_span!("2")), (8..9).into()),
        },
    );

    assert_eq!(
        no_span!("2 < 3"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("2")), (0..1).into()),
            op: BinOp::Lt,
            rhs: Spanned(Box::new(no_span!("3")), (4..5).into()),
        },
    );

    assert_eq!(
        no_span!("2 < 3 * 1"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("2")), (0..1).into()),
            op: BinOp::Lt,
            rhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(no_span!("3")), (4..5).into()),
                    op: BinOp::Mul,
                    rhs: Spanned(Box::new(no_span!("1")), (8..9).into()),
                }),
                (4..9).into(),
            ),
        },
    );

    assert_eq!(
        no_span!("2 < (3 * 1)"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("2")), (0..1).into()),
            op: BinOp::Lt,
            rhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(no_span!("3")), (5..6).into()),
                    op: BinOp::Mul,
                    rhs: Spanned(Box::new(no_span!("1")), (9..10).into()),
                }),
                (5..10).into(),
            ),
        },
    );

    assert_eq!(
        no_span!("true && false"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("true")), (0..4).into()),
            op: BinOp::And,
            rhs: Spanned(Box::new(no_span!("false")), (8..13).into()),
        },
    );

    assert_eq!(
        no_span!("true || false"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("true")), (0..4).into()),
            op: BinOp::Or,
            rhs: Spanned(Box::new(no_span!("false")), (8..13).into()),
        },
    );

    assert_eq!(
        no_span!("true && false || true"),
        Expr::Binary {
            lhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(no_span!("true")), (0..4).into()),
                    op: BinOp::And,
                    rhs: Spanned(Box::new(no_span!("false")), (8..13).into()),
                }),
                (0..13).into(),
            ),
            op: BinOp::Or,
            rhs: Spanned(Box::new(no_span!("true")), (17..21).into()),
        },
    );

    assert_eq!(
        no_span!("true || false && true"),
        Expr::Binary {
            lhs: Spanned(Box::new(no_span!("true")), (0..4).into()),
            op: BinOp::Or,
            rhs: Spanned(
                Box::new(Expr::Binary {
                    lhs: Spanned(Box::new(no_span!("false")), (8..13).into()),
                    op: BinOp::And,
                    rhs: Spanned(Box::new(no_span!("true")), (17..21).into()),
                }),
                (8..21).into(),
            ),
        },
    );
}

#[test]
fn comments() {
    assert_eq!(
        no_span!("let x = y in z // comment for let"),
        Expr::Let {
            lhs: "x",
            rhs: Spanned(Box::new(no_span!("y")), (8..9).into()),
            cont: Spanned(Box::new(no_span!("z")), (13..14).into()),
        },
    );
}

#[test]
fn dicts() {
    assert_eq!(
        no_span!("{k -> v}"),
        Expr::Dict {
            map: vec![DictEntry {
                key: Spanned(no_span!("k"), (1..2).into()),
                val: Spanned(no_span!("v"), (6..7).into()),
            }],
            hint: None,
        },
    );

    assert_eq!(
        no_span!("@hashdict {k -> v}"),
        Expr::Dict {
            map: vec![DictEntry {
                key: Spanned(no_span!("k"), (11..12).into()),
                val: Spanned(no_span!("v"), (16..17).into()),
            }],
            hint: Some(DictHint::HashDict { capacity: None }),
        },
    );

    assert_eq!(
        no_span!("@sortdict {k -> v}"),
        Expr::Dict {
            map: vec![DictEntry {
                key: Spanned(no_span!("k"), (11..12).into()),
                val: Spanned(no_span!("v"), (16..17).into()),
            }],
            hint: Some(DictHint::SortDict { capacity: None }),
        },
    );

    assert_eq!(
        no_span!("@smallvecdict {k -> v}"),
        Expr::Dict {
            map: vec![DictEntry {
                key: Spanned(no_span!("k"), (15..16).into()),
                val: Spanned(no_span!("v"), (20..21).into()),
            }],
            hint: Some(DictHint::SmallVecDict { capacity: None }),
        },
    );

    assert_eq!(
        no_span!("@vec {k -> v}"),
        Expr::Dict {
            map: vec![DictEntry {
                key: Spanned(no_span!("k"), (6..7).into()),
                val: Spanned(no_span!("v"), (11..12).into()),
            }],
            hint: Some(DictHint::Vec { capacity: None }),
        },
    );
}

#[test]
fn sets() {
    assert_eq!(no_span!("{}"), Expr::Set(vec![]));

    assert_eq!(
        no_span!("{1}"),
        Expr::Set(vec![Spanned(no_span!("1"), (1..2).into())]),
    );

    assert_eq!(
        no_span!("{x}"),
        Expr::Set(vec![Spanned(no_span!("x"), (1..2).into())]),
    );

    assert_eq!(
        no_span!("{0, 1}"),
        Expr::Set(vec![
            Spanned(no_span!("0"), (1..2).into()),
            Spanned(no_span!("1"), (4..5).into()),
        ]),
    );

    assert_eq!(
        no_span!("{x, y}"),
        Expr::Set(vec![
            Spanned(no_span!("x"), (1..2).into()),
            Spanned(no_span!("y"), (4..5).into()),
        ]),
    );
}

#[test]
fn records() {
    assert_eq!(
        no_span!("<a = 1, b = 2>"),
        Expr::Record {
            vals: vec![
                RecordValue {
                    name: "a".into(),
                    val: Spanned(no_span!("1"), (5..6).into()),
                },
                RecordValue {
                    name: "b".into(),
                    val: Spanned(no_span!("2"), (12..13).into()),
                },
            ],
        },
    );
}

#[test]
fn fields() {
    assert_eq!(
        no_span!("x.foo"),
        Expr::Field {
            expr: Spanned(Box::new(no_span!("x")), (0..1).into()),
            field: "foo".into(),
        },
    );

    assert_eq!(
        no_span!("x.foo * y.doo"),
        Expr::Binary {
            lhs: Spanned(
                Box::new(Expr::Field {
                    expr: Spanned(Box::new(no_span!("x")), (0..1).into()),
                    field: "foo".into(),
                }),
                (0..5).into(),
            ),
            op: BinOp::Mul,
            rhs: Spanned(
                Box::new(Expr::Field {
                    expr: Spanned(Box::new(no_span!("y")), (8..9).into()),
                    field: "doo".into(),
                }),
                (8..13).into(),
            ),
        },
    );

    assert_eq!(
        no_span!("x.foo.doo"),
        Expr::Field {
            expr: Spanned(
                Box::new(Expr::Field {
                    expr: Spanned(Box::new(no_span!("x")), (0..1).into()),
                    field: "foo".into(),
                }),
                (0..5).into(),
            ),
            field: "doo".into(),
        },
    );

    assert_eq!(
        no_span!("< first = 1 >.first"),
        Expr::Field {
            expr: Spanned(
                Box::new(Expr::Record {
                    vals: vec![RecordValue {
                        name: "first".into(),
                        val: Spanned(no_span!("1"), (10..11).into()),
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
    assert_eq!(
        no_span!("x(1)"),
        Expr::Get {
            lhs: Spanned(Box::new(no_span!("x")), (0..1).into()),
            rhs: Spanned(Box::new(no_span!("1")), (2..3).into()),
        },
    );

    assert_eq!(
        no_span!("x(y)"),
        Expr::Get {
            lhs: Spanned(Box::new(no_span!("x")), (0..1).into()),
            rhs: Spanned(Box::new(no_span!("y")), (2..3).into()),
        },
    );

    assert_eq!(
        no_span!("x(y)(z)"),
        Expr::Get {
            lhs: Spanned(
                Box::new(Expr::Get {
                    lhs: Spanned(Box::new(no_span!("x")), (0..1).into()),
                    rhs: Spanned(Box::new(no_span!("y")), (2..3).into()),
                }),
                (0..4).into(),
            ),
            rhs: Spanned(Box::new(no_span!("z")), (5..6).into()),
        },
    );

    // FIXME
    // assert_eq!(no_span!(
    //     "< first = 1 >(1)"),
    //     Expr::Get {
    //         lhs: Spanned(
    //             Box::new(Expr::Record {
    //                 vals: vec![RecordValue {
    //                     name: "first".into(),
    //                     val: Spanned(no_span!("1"), (10..11).into()),
    //                 }],
    //             }),
    //             (0..13).into(),
    //         ),
    //         rhs: Spanned(Box::new(no_span!("1")), (16..17).into()),
    //     },
    // );

    // TODO get on dict literal

    // TODO get on set literal
}

#[test]
fn sum() {
    assert_eq!(
        no_span!("sum(<k,v> <- X) v"),
        Expr::Sum {
            key: &"k",
            val: &"v",
            head: Spanned(Box::new(no_span!("X")), (13..14).into()),
            body: Spanned(Box::new(no_span!("v")), (16..17).into()),
        },
    );

    assert_eq!(
        no_span!("sum(<k,v> <- X) {k -> v}"),
        Expr::Sum {
            key: &"k",
            val: &"v",
            head: Spanned(Box::new(no_span!("X")), (13..14).into()),
            body: Spanned(
                Box::new(Expr::Dict {
                    map: vec![DictEntry {
                        key: Spanned(no_span!("k"), (17..18).into()),
                        val: Spanned(no_span!("v"), (22..23).into()),
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
    assert_eq!(
        no_span!("load[<foobar: @vec {int -> real}>](\"foo.csv\")"),
        Expr::Load {
            r#type: Type::Record(vec![RecordType {
                name: "foobar".into(),
                r#type: Type::Dict {
                    key: Box::new(Type::Int),
                    val: Box::new(Type::Real),
                    hint: Some(DictHint::Vec { capacity: None }),
                },
            }]),
            path: "foo.csv",
        },
    );
}

#[test]
fn concat() {
    assert_eq!(
        no_span!("concat(k,v)"),
        Expr::Concat {
            lhs: Spanned(no_span!("k"), (7..8).into()).boxed(),
            rhs: Spanned(no_span!("v"), (9..10).into()).boxed(),
        },
    );
}

// #[test]
// fn external() {
//     todo!()
// }

#[test]
fn promote() {
    assert_eq!(
        no_span!("promote[real](1)"),
        Expr::Promote {
            promo: Type::Real,
            expr: Spanned(no_span!("1"), (14..15).into()).boxed(),
        },
    );
}

#[test]
fn unique() {
    assert_eq!(
        no_span!("unique(x)"),
        Expr::Unique {
            expr: Spanned(Box::new(no_span!("x")), (7..8).into()),
        },
    );
}

#[test]
fn tpch_q3() {
    let prog = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q3.sdql"));
    let _ = no_span!(prog);
}

#[test]
fn tpch_q6() {
    let prog = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q6.sdql"));
    let _ = no_span!(prog);
}
