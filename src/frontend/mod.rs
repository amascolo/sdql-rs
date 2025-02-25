#![allow(dead_code)]
mod expr;
mod lexer;
mod tests;
mod r#type;

use chumsky::{input::ValueInput, prelude::*};
use expr::{BinaryOp, DictEntry, Expr, RecordValue, UnaryOp, Value};
use lexer::{DictHint, ScalarType, Span, Spanned, Token};
use r#type::Type;

#[allow(dead_code)]
// #[derive(Debug)]
struct Error {
    span: Span,
    msg: String,
}

impl Value<'_> {
    #[allow(dead_code)]
    fn num(self, span: Span) -> Result<f64, Error> {
        if let Value::Float(x) = self {
            Ok(x)
        } else {
            Err(Error {
                span,
                msg: format!("'{self}' is not a number"),
            })
        }
    }
}

fn expr_parser<'src, I>()
-> impl Parser<'src, I, Spanned<Expr<'src>>, extra::Err<Rich<'src, Token<'src>, Span>>> + Clone
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    recursive(|expr| {
        let inline_expr = recursive(|inline_expr| {
            let val = select! {
                Token::Bool(x) => Expr::Value(Value::Bool(x)),
                Token::Num(n) => Expr::Value(Value::Float(n)),
                Token::Str(s) => Expr::Value(Value::String(s)),
            }
            .labelled("value");

            let ident = select! { Token::Ident(ident) => ident }.labelled("identifier");

            // A let expression
            let let_ = just(Token::Let)
                .ignore_then(ident)
                .then_ignore(just(Token::Op("=")))
                .then(inline_expr)
                .then_ignore(just(Token::In))
                .then(expr.clone())
                .map(|((name, val), body)| Expr::Let {
                    lhs: name,
                    rhs: Box::new(val),
                    cont: Box::new(body),
                });

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
                    Expr::Record(
                        v.into_iter()
                            .map(|(name, val @ (_, span))| RecordValue {
                                name: (name.into(), (span.start - 4..span.end - 4).into()), // FIXME
                                val,
                            })
                            .collect(),
                    )
                })
                .delimited_by(just(Token::Op("<")), just(Token::Op(">")));

            let atom = val
                .or(ident.map(Expr::Sym))
                .or(let_)
                .or(dict)
                .or(record)
                .map_with(|expr, e| (expr, e.span()))
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))))
                .boxed();

            let neg = just(Token::Op("-"))
                .repeated()
                .foldr(atom, |_op, rhs @ (_, span)| {
                    (
                        Expr::Unary {
                            op: UnaryOp::Neg,
                            expr: Box::new(rhs),
                        },
                        (span.start - 1..span.end).into(),
                    )
                });

            let field = neg
                .clone()
                .then(just(Token::Ctrl('.')).ignore_then(ident).or_not())
                .map_with(|(expr, field), e| match field {
                    None => expr,
                    Some(field) => (
                        Expr::Field {
                            expr: Box::new(expr),
                            field: field.into(),
                        },
                        e.span(),
                    ),
                });

            let not = just(Token::Op("!"))
                .repeated()
                .foldr(field, |_op, rhs @ (_, span)| {
                    (
                        Expr::Unary {
                            op: UnaryOp::Not,
                            expr: Box::new(rhs),
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
                .foldl_with(op.then(not).repeated(), |a, (op, b), e| {
                    (
                        Expr::Binary {
                            lhs: Box::new(a),
                            op,
                            rhs: Box::new(b),
                        },
                        e.span(),
                    )
                });

            // Sum ops (add and subtract) have equal precedence
            let op = just(Token::Op("+"))
                .to(BinaryOp::Add)
                .or(just(Token::Op("-")).to(BinaryOp::Sub));
            let sum = product
                .clone()
                .foldl_with(op.then(product).repeated(), |a, (op, b), e| {
                    (
                        Expr::Binary {
                            lhs: Box::new(a),
                            op,
                            rhs: Box::new(b),
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
                .foldl_with(op.then(sum).repeated(), |a, (op, b), e| {
                    (
                        Expr::Binary {
                            lhs: Box::new(a),
                            op,
                            rhs: Box::new(b),
                        },
                        e.span(),
                    )
                });

            let op = just(Token::Op("&&")).to(BinaryOp::And);
            let and = compare
                .clone()
                .foldl_with(op.then(compare).repeated(), |a, (op, b), e| {
                    (
                        Expr::Binary {
                            lhs: Box::new(a),
                            op,
                            rhs: Box::new(b),
                        },
                        e.span(),
                    )
                });

            let op = just(Token::Op("||")).to(BinaryOp::Or);
            let or = and
                .clone()
                .foldl_with(op.then(and).repeated(), |a, (op, b), e| {
                    (
                        Expr::Binary {
                            lhs: Box::new(a),
                            op,
                            rhs: Box::new(b),
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
                .map_with(|((cond, a), b), e| {
                    (
                        Expr::If {
                            r#if: Box::new(cond),
                            then: Box::new(a),
                            r#else: b.map(Box::new),
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
                (
                    Expr::Sum {
                        key: Box::new(key),
                        val: Box::new(val),
                        head: Box::new(head),
                        body: Box::new(body),
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

            let record_type = type_
                .clone()
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(just(Token::Op("<")), just(Token::Op(">")))
                .map(|v| Type::Record(v));

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
            .map_with(|(r#type, path), e| (Expr::Load { r#type, path }, e.span()));

        inline_expr.or(if_).or(sum).or(load)
    })
}
