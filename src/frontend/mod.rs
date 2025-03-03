#![allow(dead_code)]
pub mod lexer;
mod tests;

use crate::ir::expr::{BinaryOp, DictEntry, Expr, RecordValue, UnaryOp};
use crate::ir::r#type::{DictHint, RecordType, Type};
use crate::runtime::Date;
use chumsky::{input::ValueInput, prelude::*};
use lexer::{ScalarType, Spanned, Token};
use time::format_description::well_known::Iso8601;

#[allow(dead_code)]
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
        let hint = just(Token::At)
            .ignore_then(
                just(Token::DictHint(DictHint::HashDict))
                    .or(just(Token::DictHint(DictHint::SortDict)))
                    .or(just(Token::DictHint(DictHint::SmallVecDict)))
                    .or(just(Token::DictHint(DictHint::Vec))),
            )
            .map(|hint| match hint {
                Token::DictHint(hint) => hint,
                _ => unreachable!(),
            })
            .boxed();

        let varchar_type = just(Token::Type(ScalarType::VarChar))
            .ignore_then(
                select! { Token::Integer(n) => n }
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
            )
            .boxed();

        let type_ = recursive(|type_| {
            let varchar = varchar_type
                .clone()
                .map(|n| Type::String { max_len: Some(n) });

            let scalar = choice((
                varchar,
                just(Token::Type(ScalarType::String)).to(Type::String { max_len: None }),
                just(Token::Type(ScalarType::Bool)).to(Type::Bool),
                just(Token::Type(ScalarType::Date)).to(Type::Date),
                just(Token::Type(ScalarType::Int)).to(Type::Int),
                just(Token::Type(ScalarType::Long)).to(Type::Long),
                just(Token::Type(ScalarType::Real)).to(Type::Real),
            ));

            let record_type = select! { Token::Ident(ident) => ident }
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

            let dict_type = hint
                .clone()
                .or_not()
                .then(
                    type_
                        .clone()
                        .then_ignore(just(Token::Arrow("->")))
                        .then(type_.clone())
                        .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))),
                )
                .map(|(hint, (key, val))| Type::Dict {
                    key: Box::new(key),
                    val: Box::new(val),
                    hint,
                });

            scalar.or(record_type).or(dict_type)
        });

        let inline_expr = recursive(|_inline_expr| {
            let val = select! {
                Token::Bool(val) => Expr::Bool { val },
                Token::Integer(val) => Expr::Int { val: val.try_into().unwrap() },
                Token::Real(val) => Expr::Real { val },
                Token::Str(val) => Expr::String { val, max_len: None },
            }
            .labelled("value");

            let long = just(Token::At)
                .then(just(Token::Type(ScalarType::Long)))
                .ignore_then(select! { Token::Integer(n) => n })
                .map(|val| Expr::Long { val });

            let varchar = just(Token::At)
                .ignore_then(varchar_type)
                .then(select! { Token::Str(s) => s })
                .map(|(n, val)| Expr::String {
                    val,
                    max_len: Some(n),
                });

            let date = just(Token::Type(ScalarType::Date))
                .ignore_then(
                    select! { Token::Integer(n) => n }
                        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                )
                .map(|val| Expr::Date {
                    // TODO tokens should be &str only - avoid String allocation here
                    val: Date::new(time::Date::parse(&val.to_string(), &Iso8601::DEFAULT).unwrap()),
                });

            let sym = select! { Token::Ident(ident) => ident }.map(|val| Expr::Sym { val });

            let dict_items = expr
                .clone()
                .then_ignore(just(Token::Arrow("->")))
                .then(expr.clone())
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>();

            let dict = hint
                .or_not()
                .then(dict_items.delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))))
                .map(|(hint, v)| Expr::Dict {
                    map: v
                        .into_iter()
                        .map(|(key, val)| DictEntry { key, val })
                        .collect(),
                    hint,
                });

            let set_items = expr
                .clone()
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>();

            let set = set_items
                .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
                .map(Expr::Set);

            let record_items = select! { Token::Ident(ident) => ident }
                .then_ignore(just(Token::Op("=")))
                .then(expr.clone())
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>();

            let record = record_items
                .clone()
                .map(|v| Expr::Record {
                    vals: v
                        .into_iter()
                        .map(|(name, val)| RecordValue {
                            name: name.into(),
                            val,
                        })
                        .collect(),
                })
                .delimited_by(just(Token::Op("<")), just(Token::Op(">")));

            let load = just(Token::Load)
                .ignore_then(type_.delimited_by(just(Token::Ctrl('[')), just(Token::Ctrl(']'))))
                .then(
                    select! { Token::Str(s) => s }
                        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                )
                .map(|(r#type, path)| Expr::Load { r#type, path });

            let atom = val
                .or(long)
                .or(varchar)
                .or(date)
                .or(sym)
                .or(dict)
                .or(set)
                .or(record)
                .or(load)
                .map_with(|expr, e| Spanned(expr, e.span()))
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))))
                .boxed();

            let field = atom
                .clone()
                .foldl_with(
                    just(Token::Ctrl('.'))
                        .ignore_then(select! { Token::Ident(ident) => ident })
                        .repeated(),
                    |expr, field, e| {
                        Spanned(
                            Expr::Field {
                                expr: expr.boxed(),
                                field: field.into(),
                            },
                            e.span(),
                        )
                    },
                )
                .boxed();

            let get = field
                .clone()
                .foldl_with(
                    expr.clone()
                        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')')))
                        .repeated(),
                    |lhs, rhs, e| {
                        Spanned(
                            Expr::Get {
                                lhs: lhs.boxed(),
                                rhs: rhs.boxed(),
                            },
                            e.span(),
                        )
                    },
                )
                .boxed();

            let neg = just(Token::Op("-"))
                .repeated()
                .foldr(get, |_op, rhs| {
                    let Spanned(_, span) = rhs;
                    {
                        Spanned(
                            Expr::Unary {
                                op: UnaryOp::Neg,
                                expr: rhs.boxed(),
                            },
                            (span.start - 1..span.end).into(),
                        )
                    }
                })
                .boxed();

            let not = just(Token::Op("!"))
                .repeated()
                .foldr(neg, |_op, rhs| {
                    let Spanned(_, span) = rhs;
                    Spanned(
                        Expr::Unary {
                            op: UnaryOp::Not,
                            expr: rhs.boxed(),
                        },
                        (span.start - 1..span.end).into(),
                    )
                })
                .boxed();

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
                select! { Token::Ident(ident) => ident }
                    .clone()
                    .then(
                        just(Token::Ctrl(','))
                            .ignore_then(select! { Token::Ident(ident) => ident }),
                    )
                    .delimited_by(just(Token::Op("<")), just(Token::Op(">")))
                    .then(just(Token::Arrow("<-")).ignore_then(expr.clone()))
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
            )
            .then(expr.clone())
            .map_with(|(((key, val), head), body), e| {
                Spanned(
                    Expr::Sum {
                        key,
                        val,
                        head: head.boxed(),
                        body: body.boxed(),
                    },
                    e.span(),
                )
            });

        let let_ = just(Token::Let)
            .ignore_then(select! { Token::Ident(ident) => ident })
            .then_ignore(just(Token::Op("=")))
            .then(expr.clone())
            .then_ignore(just(Token::In).or_not())
            .then(expr.clone())
            .map_with(|((name, val), body), e| {
                Spanned(
                    Expr::Let {
                        lhs: name,
                        rhs: val.boxed(),
                        cont: body.boxed(),
                    },
                    e.span(),
                )
            });

        inline_expr.or(if_).or(sum).or(let_)
    })
}
