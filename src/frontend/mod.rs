mod lexer;

use chumsky::{input::ValueInput, prelude::*};

use lexer::{Span, Spanned, Token};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum Value<'src> {
    Null,
    Bool(bool),
    Num(f64),
    Str(&'src str),
    List(Vec<Self>),
}

#[allow(dead_code)]
// #[derive(Debug)]
struct Error {
    span: Span,
    msg: String,
}

impl Value<'_> {
    #[allow(dead_code)]
    fn num(self, span: Span) -> Result<f64, Error> {
        if let Value::Num(x) = self {
            Ok(x)
        } else {
            Err(Error {
                span,
                msg: format!("'{}' is not a number", self),
            })
        }
    }
}

impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(x) => write!(f, "{}", x),
            Self::Num(x) => write!(f, "{}", x),
            Self::Str(x) => write!(f, "{}", x),
            Self::List(xs) => write!(
                f,
                "[{}]",
                xs.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            // Self::Func(name) => write!(f, "<function: {}>", name),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Less,
    Great,
    LessEq,
    GreatEq,
    And,
    Or,
}

// An expression node in the AST. Children are spanned so we can generate useful runtime errors.
#[derive(Clone, Debug, PartialEq)]
enum Expr<'src> {
    Error,
    Value(Value<'src>),
    Record(Vec<(Spanned<Self>, Spanned<Self>)>),
    Local(&'src str),
    Let(&'src str, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Then(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Not(Box<Spanned<Self>>),
    Neg(Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
    If(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),
}

fn expr_parser<'src, I>()
-> impl Parser<'src, I, Spanned<Expr<'src>>, extra::Err<Rich<'src, Token<'src>, Span>>> + Clone
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    recursive(|expr| {
        let inline_expr = recursive(|inline_expr| {
            let val = select! {
                Token::Null => Expr::Value(Value::Null),
                Token::Bool(x) => Expr::Value(Value::Bool(x)),
                Token::Num(n) => Expr::Value(Value::Num(n)),
                Token::Str(s) => Expr::Value(Value::Str(s)),
            }
            .labelled("value");

            let ident = select! { Token::Ident(ident) => ident }.labelled("identifier");

            // A let expression
            let let_ = just(Token::Let)
                .ignore_then(ident)
                .then_ignore(just(Token::Op("=")))
                .then(inline_expr)
                // .then_ignore(just(Token::Ctrl(';')))
                .then_ignore(just(Token::In))
                .then(expr.clone())
                .map(|((name, val), body)| Expr::Let(name, Box::new(val), Box::new(body)));

            // A list of expressions
            let items = expr
                .clone()
                .then_ignore(just(Token::Op("=")))
                .then(expr.clone())
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>();

            let record = items
                .clone()
                .map(Expr::Record)
                .delimited_by(just(Token::Op("<")), just(Token::Op(">")));

            // 'Atoms' are expressions that contain no ambiguity
            let atom = val
                .or(ident.map(Expr::Local))
                .or(let_)
                .or(record)
                // In Nano Rust, `print` is just a keyword, just like Python 2, for simplicity
                // .or(just(Token::Print)
                //     .ignore_then(
                //         expr.clone()
                //             .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                //     )
                //     .map(|expr| Expr::Print(Box::new(expr))))
                .map_with(|expr, e| (expr, e.span()))
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))))
                // Attempt to recover anything that looks like a parenthesised expression but contains errors
                .recover_with(via_parser(nested_delimiters(
                    Token::Ctrl('('),
                    Token::Ctrl(')'),
                    [
                        (Token::Ctrl('['), Token::Ctrl(']')),
                        (Token::Ctrl('{'), Token::Ctrl('}')),
                    ],
                    |span| (Expr::Error, span),
                )))
                // Attempt to recover anything that looks like a list but contains errors
                .recover_with(via_parser(nested_delimiters(
                    Token::Ctrl('['),
                    Token::Ctrl(']'),
                    [
                        (Token::Ctrl('('), Token::Ctrl(')')),
                        (Token::Ctrl('{'), Token::Ctrl('}')),
                    ],
                    |span| (Expr::Error, span),
                )))
                .boxed();

            let neg = just(Token::Op("-"))
                .repeated()
                .foldr(atom, |_op, rhs @ (_, span)| {
                    (Expr::Neg(Box::new(rhs)), (span.start - 1..span.end).into())
                });

            let not = just(Token::Op("!"))
                .repeated()
                .foldr(neg, |_op, rhs @ (_, span)| {
                    (Expr::Not(Box::new(rhs)), (span.start - 1..span.end).into())
                });

            // Product ops (multiply and divide) have equal precedence
            let op = just(Token::Op("*"))
                .to(BinaryOp::Mul)
                .or(just(Token::Op("/")).to(BinaryOp::Div));
            let product = not
                .clone()
                .foldl_with(op.then(not).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                });

            // Sum ops (add and subtract) have equal precedence
            let op = just(Token::Op("+"))
                .to(BinaryOp::Add)
                .or(just(Token::Op("-")).to(BinaryOp::Sub));
            let sum = product
                .clone()
                .foldl_with(op.then(product).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                });

            // Comparison ops (equal, not-equal, etc) have equal precedence
            let op = choice((
                just(Token::Op("==")).to(BinaryOp::Eq),
                just(Token::Op("!=")).to(BinaryOp::NotEq),
                just(Token::Op("<=")).to(BinaryOp::LessEq),
                just(Token::Op(">=")).to(BinaryOp::GreatEq),
                just(Token::Op("<")).to(BinaryOp::Less),
                just(Token::Op(">")).to(BinaryOp::Great),
            ));
            let compare = sum
                .clone()
                .foldl_with(op.then(sum).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                });

            let op = just(Token::Op("&&")).to(BinaryOp::And);
            let and = compare
                .clone()
                .foldl_with(op.then(compare).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                });

            let op = just(Token::Op("||")).to(BinaryOp::Or);
            let or = and
                .clone()
                .foldl_with(op.then(and).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                });

            or.labelled("expression").as_context()
        });

        // Blocks are expressions but delimited with braces
        let block = expr
            .clone()
            .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
            // Attempt to recover anything that looks like a block but contains errors
            .recover_with(via_parser(nested_delimiters(
                Token::Ctrl('{'),
                Token::Ctrl('}'),
                [
                    (Token::Ctrl('('), Token::Ctrl(')')),
                    (Token::Ctrl('['), Token::Ctrl(']')),
                ],
                |span| (Expr::Error, span),
            )));

        let if_ = recursive(|if_| {
            just(Token::If)
                .ignore_then(expr.clone())
                .then(expr.clone()) // TODO expr was block
                .then(
                    just(Token::Else)
                        .ignore_then(expr.clone().or(if_)) // TODO expr was block
                        .or_not(),
                )
                .map_with(|((cond, a), b), e| {
                    (
                        Expr::If(
                            Box::new(cond),
                            Box::new(a),
                            // If an `if` expression has no trailing `else` block, we magic up one that just produces null
                            Box::new(b.unwrap_or_else(|| (Expr::Value(Value::Null), e.span()))),
                        ),
                        e.span(),
                    )
                })
        });

        // Both blocks and `if` are 'block expressions' and can appear in the place of statements
        let block_expr = block.or(if_);

        let block_chain = block_expr
            .clone()
            .foldl_with(block_expr.clone().repeated(), |a, b, e| {
                (Expr::Then(Box::new(a), Box::new(b)), e.span())
            });

        let block_recovery = nested_delimiters(
            Token::Ctrl('{'),
            Token::Ctrl('}'),
            [
                (Token::Ctrl('('), Token::Ctrl(')')),
                (Token::Ctrl('['), Token::Ctrl(']')),
            ],
            |span| (Expr::Error, span),
        );

        block_chain
            .labelled("block")
            // Expressions, chained by semicolons, are statements
            .or(inline_expr.clone())
            .recover_with(skip_then_retry_until(
                block_recovery.ignored().or(any().ignored()),
                one_of([
                    Token::Ctrl(';'),
                    Token::Ctrl('}'),
                    Token::Ctrl(')'),
                    Token::Ctrl(']'),
                ])
                .ignored(),
            ))
            .foldl_with(
                just(Token::Ctrl(';')).ignore_then(expr.or_not()).repeated(),
                |a, b, e| {
                    let span: Span = e.span();
                    (
                        Expr::Then(
                            Box::new(a),
                            // If there is no b expression then its span is the end of the statement/block.
                            Box::new(
                                b.unwrap_or_else(|| (Expr::Value(Value::Null), span.to_end())),
                            ),
                        ),
                        span,
                    )
                },
            )
    })
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lexer;

    fn check_expr(src: &str, exp: Expr) {
        let (tokens, _errs) = lexer().parse(src).into_output_errors();

        let tokens = tokens.unwrap();
        // for (t, _span) in &tokens {
        //     println!("{t}");
        // }
        // assert!(errs.is_empty());

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

    // FIXME
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

        // check_expr(
        //     "if (!true) then (0) else (1)",
        //     Expr::If(
        //         Box::new((Expr::Value(Value::Bool(true)), (5..9).into())),
        //         Box::new((Expr::Value(Value::Num(0f64)), (17..18).into())),
        //         Box::new((Expr::Value(Value::Num(1f64)), (26..27).into())),
        //     ),
        // );
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
    fn records() {
        check_expr(
            "<k = 1, v = 2>",
            Expr::Record(vec![
                (
                    (Expr::Local("k"), (1..2).into()),
                    (Expr::Value(Value::Num(1f64)), (5..6).into()),
                ),
                (
                    (Expr::Local("v"), (8..9).into()),
                    (Expr::Value(Value::Num(2f64)), (12..13).into()),
                ),
            ]),
        )
    }
}
