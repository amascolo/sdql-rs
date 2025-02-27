#![allow(dead_code)]
pub mod lexer;
mod tests;

use crate::ir::expr::{BinaryOp, DictEntry, Expr, RecordValue, UnaryOp};
use crate::ir::r#type::{DictHint, RecordType, Type};
use chumsky::{input::ValueInput, prelude::*};
use lexer::{ScalarType, Spanned, Token};

#[allow(dead_code)]
// #[derive(Debug)]
struct Error {
    span: SimpleSpan,
    msg: String,
}

fn expr_parser<'src, I>()
-> impl Parser<'src, I, Spanned<Expr<'src>>, extra::Err<Rich<'src, Token<'src>, SimpleSpan>>> + Clone
where
    I: ValueInput<'src, Token = Token<'src>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let inline_expr = recursive(|inline_expr| {
            let val = select! {
                Token::Bool(x) => Expr::Bool{val: x},
                Token::Num(n) => Expr::Real{val: n},
                Token::Str(s) => Expr::String{val:s},
            }
            .labelled("value");

            let ident = select! { Token::Ident(ident) => ident }.labelled("identifier");

            let let_ = just(Token::Let)
                .ignore_then(ident)
                .then_ignore(just(Token::Op("=")))
                .then(inline_expr)
                .then_ignore(just(Token::In))
                .then(expr.clone())
                .map(
                    |((name, val), body): ((_, Spanned<Expr>), Spanned<Expr>)| Expr::Let {
                        lhs: name,
                        rhs: val.boxed(),
                        cont: body.boxed(),
                    },
                );

            let dict_items = expr
                .clone()
                .then_ignore(just(Token::Arrow("->")))
                .then(expr.clone())
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>();

            let hint = just(Token::At)
                .ignore_then(
                    just(Token::DictHint(DictHint::HashDict))
                        .or(just(Token::DictHint(DictHint::SortDict)))
                        .or(just(Token::DictHint(DictHint::SmallVecDict)))
                        .or(just(Token::DictHint(DictHint::Vec))),
                )
                .boxed();

            let dict = hint
                .or_not()
                .then(dict_items.delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))))
                .map(|(hint, v)| Expr::Dict {
                    map: v
                        .into_iter()
                        .map(|(key, val)| DictEntry { key, val })
                        .collect(),
                    hint: hint.map(|hint| match hint {
                        Token::DictHint(hint) => hint,
                        _ => unreachable!(),
                    }),
                });

            let record_items = ident
                .then_ignore(just(Token::Op("=")))
                .then(expr.clone())
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>();

            let record = record_items
                .clone()
                .map(|v| {
                    Expr::Record {
                        vals: v
                            .into_iter()
                            .map(|(name, val)| {
                                let Spanned(_, span) = val;
                                RecordValue {
                                    name: Spanned(
                                        name.into(),
                                        // FIXME hardcoded span
                                        (span.start - 4..span.end - 4).into(),
                                    ),
                                    val,
                                }
                            })
                            .collect(),
                    }
                })
                .delimited_by(just(Token::Op("<")), just(Token::Op(">")));

            let atom = val
                .or(ident.map(|val| Expr::Sym { val }))
                .or(let_)
                .or(dict)
                .or(record)
                .map_with(|expr, e| Spanned(expr, e.span()))
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))))
                .boxed();

            let neg = just(Token::Op("-")).repeated().foldr(atom, |_op, rhs| {
                let Spanned(_, span) = rhs;
                {
                    Spanned(
                        Expr::Unary {
                            op: UnaryOp::Neg,
                            expr: rhs.boxed(),
                        },
                        // FIXME hardcoded span
                        (span.start - 1..span.end).into(),
                    )
                }
            });

            let field = neg
                .clone()
                .then(just(Token::Ctrl('.')).ignore_then(ident).or_not())
                .map_with(|(expr, field), e| match field {
                    None => expr,
                    Some(field) => Spanned(
                        Expr::Field {
                            expr: expr.boxed(),
                            field: field.into(),
                        },
                        e.span(),
                    ),
                });

            let not = just(Token::Op("!")).repeated().foldr(field, |_op, rhs| {
                let Spanned(_, span) = rhs;
                Spanned(
                    Expr::Unary {
                        op: UnaryOp::Not,
                        expr: rhs.boxed(),
                    },
                    (span.start - 1..span.end).into(),
                )
            });

            // Product ops (multiply and divide) have equal precedence
            let op = just(Token::Op("*"))
                .to(BinaryOp::Mul)
                .or(just(Token::Op("/")).to(BinaryOp::Div));
            let product = not
                .clone()
                .foldl_with(op.then(not).repeated(), |lhs, (op, rhs), e| {
                    Spanned(
                        Expr::Binary {
                            lhs: lhs.boxed(),
                            op,
                            rhs: rhs.boxed(),
                        },
                        e.span(),
                    )
                });

            // Sum ops (add and subtract) have equal precedence
            let op = just(Token::Op("+"))
                .to(BinaryOp::Add)
                .or(just(Token::Op("-")).to(BinaryOp::Sub));
            let sum =
                product
                    .clone()
                    .foldl_with(op.then(product).repeated(), |lhs, (op, rhs), e| {
                        Spanned(
                            Expr::Binary {
                                lhs: lhs.boxed(),
                                op,
                                rhs: rhs.boxed(),
                            },
                            e.span(),
                        )
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
                .foldl_with(op.then(sum).repeated(), |lhs, (op, rhs), e| {
                    Spanned(
                        Expr::Binary {
                            lhs: lhs.boxed(),
                            op,
                            rhs: rhs.boxed(),
                        },
                        e.span(),
                    )
                });

            let op = just(Token::Op("&&")).to(BinaryOp::And);
            let and =
                compare
                    .clone()
                    .foldl_with(op.then(compare).repeated(), |lhs, (op, rhs), e| {
                        Spanned(
                            Expr::Binary {
                                lhs: lhs.boxed(),
                                op,
                                rhs: rhs.boxed(),
                            },
                            e.span(),
                        )
                    });

            let op = just(Token::Op("||")).to(BinaryOp::Or);
            let or = and
                .clone()
                .foldl_with(op.then(and).repeated(), |lhs, (op, rhs), e| {
                    Spanned(
                        Expr::Binary {
                            lhs: lhs.boxed(),
                            op,
                            rhs: rhs.boxed(),
                        },
                        e.span(),
                    )
                });

            or.labelled("expression").as_context()
        });

        let if_ = recursive(|if_| {
            just(Token::If)
                .ignore_then(expr.clone())
                .then(just(Token::Then).ignore_then(expr.clone()))
                .then(just(Token::Else).ignore_then(expr.clone().or(if_)).or_not())
                .map_with(|((r#if, then), r#else), e| {
                    Spanned(
                        Expr::If {
                            r#if: r#if.boxed(),
                            then: then.boxed(),
                            r#else: r#else.map(Spanned::boxed),
                        },
                        e.span(),
                    )
                })
        });

        let sum = just(Token::Sum)
            .ignore_then(
                inline_expr
                    .clone()
                    .then(just(Token::Ctrl(',')).ignore_then(inline_expr.clone()))
                    .delimited_by(just(Token::Op("<")), just(Token::Op(">")))
                    .then(just(Token::Arrow("<-")).ignore_then(inline_expr.clone()))
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
            )
            .then(inline_expr.clone())
            .map_with(|(((key, val), head), body), e| {
                Spanned(
                    Expr::Sum {
                        key: key.boxed(),
                        val: val.boxed(),
                        head: head.boxed(),
                        body: body.boxed(),
                    },
                    e.span(),
                )
            });

        let str_select = select! { Token::Str(s) => s }.labelled("str");

        let type_ = recursive(|type_| {
            let scalar = choice((
                just(Token::Type(ScalarType::String)).to(Type::String { max_len: None }),
                just(Token::Type(ScalarType::Bool)).to(Type::Bool),
                just(Token::Type(ScalarType::Int)).to(Type::Int),
                just(Token::Type(ScalarType::Long)).to(Type::Long),
            ));

            let ident = select! { Token::Ident(ident) => ident }.labelled("identifier"); // TODO

            let record_type = ident
                .then_ignore(just(Token::Ctrl(':')))
                .then(type_.clone())
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(just(Token::Op("<")), just(Token::Op(">")))
                .map(|v| {
                    Type::Record(
                        v.into_iter()
                            .map(|(name, r#type)| RecordType {
                                name: name.into(),
                                r#type,
                            })
                            .collect(),
                    )
                });

            let dict_type = type_
                .clone()
                .then_ignore(just(Token::Arrow("->")))
                .then(type_.clone())
                .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
                .map(|(key, val)| Type::Dict {
                    key: Box::new(key),
                    val: Box::new(val),
                    hint: None,
                });

            scalar.or(record_type).or(dict_type)
        });

        let load = just(Token::Load)
            .ignore_then(type_.delimited_by(just(Token::Ctrl('[')), just(Token::Ctrl(']'))))
            .then(str_select.delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))))
            .map_with(|(r#type, path), e| Spanned(Expr::Load { r#type, path }, e.span()));

        inline_expr.or(if_).or(sum).or(load)
    })
}
