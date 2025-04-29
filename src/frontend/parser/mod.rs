use crate::frontend::lexer::{DictHintToken, ScalarType, Spanned, Token};
use crate::ir::expr::{BinOp, DictEntry, Expr, RecordValue, UnaryOp};
use crate::ir::r#type::{DictHint, Field, RecordType, Type};
use chumsky::error::Rich;
use chumsky::{input::ValueInput, prelude::*};
use sdql_runtime::Date;
use time::format_description::well_known::Iso8601;

#[cfg(test)]
mod tests;

pub(super) fn expr_parser<'src, I>()
-> impl Parser<'src, I, Spanned<Expr<'src>>, extra::Err<Rich<'src, Token<'src>, SimpleSpan>>> + Clone
where
    I: ValueInput<'src, Token = Token<'src>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let hint = just(Token::At)
            .ignore_then(select! { Token::DictHint(x) => x })
            .then(
                select! { Token::Integer(n) => n }
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')')))
                    .or_not(),
            )
            .map(|(hint, capacity)| {
                let capacity = capacity.map(|c| c.try_into().unwrap());
                match hint {
                    DictHintToken::HashDict => DictHint::HashDict { capacity },
                    DictHintToken::SmallVecDict => DictHint::SmallVecDict { capacity },
                    DictHintToken::SortDict => DictHint::SortDict { capacity },
                    DictHintToken::Vec => DictHint::Vec { capacity },
                    DictHintToken::VecDict => DictHint::VecDict { capacity },
                }
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

            let unique = just(Token::Unique)
                .ignore_then(
                    expr.clone()
                        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                )
                .map(Spanned::boxed)
                .map(|expr| Expr::Unique { expr });

            let dom = just(Token::Dom)
                .ignore_then(
                    expr.clone()
                        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                )
                .map(Spanned::boxed)
                .map(|expr| Expr::Dom { expr });

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

            let anonymous_items = expr
                .clone()
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>();

            let set = anonymous_items
                .clone()
                .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
                .map(Expr::Set);

            let anonymous_record = anonymous_items
                .map(|v| Expr::Record {
                    vals: v
                        .into_iter()
                        .map(|val| RecordValue {
                            name: "_".into(),
                            val,
                        })
                        .collect(),
                })
                .delimited_by(just(Token::Op("<")), just(Token::Op(">")));

            let record_items = select! { Token::Ident(ident) => ident }
                .then_ignore(just(Token::Op("=")))
                .then(expr.clone())
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>();

            let record = record_items
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
                .ignore_then(
                    type_
                        .clone()
                        .delimited_by(just(Token::Ctrl('[')), just(Token::Ctrl(']'))),
                )
                .then(
                    select! { Token::Str(s) => s }
                        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                )
                .map(|(r#type, path)| Expr::Load { r#type, path });

            let range = just(Token::Range)
                .ignore_then(
                    expr.clone()
                        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                )
                .map(|expr| Expr::Range { expr: expr.boxed() });

            let args = expr
                .clone()
                .separated_by(just(Token::Ctrl(',')))
                .collect::<Vec<_>>();
            let external = just(Token::Ext)
                .ignore_then(
                    select! { Token::Backtick(s) => s }
                        .then(just(Token::Ctrl(',')).ignore_then(args))
                        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                )
                .map(|(ext, args)| Expr::External {
                    func: ext
                        .parse()
                        .expect(&format!("missing external enum variant `{ext}`")),
                    args,
                });

            let atom = val
                .or(long)
                .or(varchar)
                .or(date)
                .or(unique)
                .or(dom)
                .or(sym)
                .or(dict)
                .or(set)
                .or(anonymous_record)
                .or(record)
                .or(load)
                .or(range)
                .or(external)
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
                .to(BinOp::Mul)
                .or(just(Token::Op("/")).to(BinOp::Div));
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
                .to(BinOp::Add)
                .or(just(Token::Op("-")).to(BinOp::Sub));
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
                just(Token::Op("==")).to(BinOp::Eq),
                just(Token::Op("!=")).to(BinOp::Ne),
                just(Token::Op("<=")).to(BinOp::Le),
                just(Token::Op(">=")).to(BinOp::Ge),
                just(Token::Op("<")).to(BinOp::Lt),
                just(Token::Op(">")).to(BinOp::Gt),
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

            let op = just(Token::Op("&&")).to(BinOp::And);
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

            let op = just(Token::Op("||")).to(BinOp::Or);
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

        let idents = select! { Token::Ident(ident) => ident }
            .separated_by(just(Token::Ctrl(',')))
            .allow_trailing()
            .collect::<Vec<_>>();

        let decat = just(Token::Let)
            .ignore_then(idents.delimited_by(just(Token::Op("<")), just(Token::Op(">"))))
            .then_ignore(just(Token::Op("=")))
            .then(expr.clone())
            .then_ignore(just(Token::In).or_not())
            .then(expr.clone())
            .map_with(|((vec, val), body), e| {
                Spanned(
                    Expr::Decat {
                        lhs: vec.into_iter().map(Field::from).collect(),
                        rhs: val.boxed(),
                        cont: body.boxed(),
                    },
                    e.span(),
                )
            });

        let concat = just(Token::Concat)
            .ignore_then(
                expr.clone()
                    .then(just(Token::Ctrl(',')).ignore_then(expr.clone()))
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
            )
            .map_with(|(lhs, rhs), e| {
                Spanned(
                    Expr::Concat {
                        lhs: lhs.boxed(),
                        rhs: rhs.boxed(),
                    },
                    e.span(),
                )
            });

        let promote = just(Token::Promote)
            .ignore_then(type_.delimited_by(just(Token::Ctrl('[')), just(Token::Ctrl(']'))))
            .then(
                expr.clone()
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
            )
            .map_with(|(promo, expr), e| {
                Spanned(
                    Expr::Promote {
                        promo,
                        expr: expr.boxed(),
                    },
                    e.span(),
                )
            });

        inline_expr
            .or(if_)
            .or(sum)
            .or(let_)
            .or(decat)
            .or(concat)
            .or(promote)
            .boxed()
    })
}
