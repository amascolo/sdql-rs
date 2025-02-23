mod lexer;
mod tests;

use chumsky::{input::ValueInput, prelude::*};

use crate::frontend::lexer::DictHint;
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
    Record(Vec<Pair<'src>>),
    Dict(Dict<'src>),
    Local(&'src str),
    Let(&'src str, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Not(Box<Spanned<Self>>),
    Neg(Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
    If(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Sum(Box<Sum<'src>>),
}

#[derive(Clone, Debug, PartialEq, Default)]
struct Dict<'src> {
    map: Vec<Pair<'src>>,
    hint: Option<DictHint>,
}

#[derive(Clone, Debug, PartialEq)]
struct Pair<'src> {
    key: Spanned<Expr<'src>>,
    value: Spanned<Expr<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
struct Sum<'src> {
    key: Spanned<Expr<'src>>,
    value: Spanned<Expr<'src>>,
    head: Spanned<Expr<'src>>,
    body: Spanned<Expr<'src>>,
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
                .then_ignore(just(Token::In))
                .then(expr.clone())
                .map(|((name, val), body)| Expr::Let(name, Box::new(val), Box::new(body)));

            let dict_items = expr
                .clone()
                .then_ignore(just(Token::Arrow("->")))
                .then(expr.clone())
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>();

            let hint = just(Token::At).ignore_then(
                just(Token::DictHint(DictHint::HashDict))
                    .or(just(Token::DictHint(DictHint::SortDict)))
                    .or(just(Token::DictHint(DictHint::SmallVecDict)))
                    .or(just(Token::DictHint(DictHint::Vec))),
            );

            let dict = hint
                .or_not()
                .then(dict_items.delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))))
                .map(|(hint, v)| {
                    Expr::Dict(Dict {
                        map: v
                            .into_iter()
                            .map(|(key, value)| Pair { key, value })
                            .collect(),
                        hint: hint.map(|hint| match hint {
                            Token::DictHint(hint) => hint,
                            _ => unreachable!(),
                        }),
                    })
                });

            let record_items = expr
                .clone()
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
                            .map(|(key, value)| Pair { key, value })
                            .collect(),
                    )
                })
                .delimited_by(just(Token::Op("<")), just(Token::Op(">")));

            // 'Atoms' are expressions that contain no ambiguity
            let atom = val
                .or(ident.map(Expr::Local))
                .or(let_)
                .or(dict)
                .or(record)
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

        let if_ = recursive(|if_| {
            just(Token::If)
                .ignore_then(expr.clone())
                .then(just(Token::Then).ignore_then(expr.clone()))
                .then(just(Token::Else).ignore_then(expr.clone().or(if_)).or_not())
                .map_with(|((cond, a), b), e| {
                    (
                        Expr::If(
                            Box::new(cond),
                            Box::new(a),
                            Box::new(b.unwrap_or_else(|| (Expr::Value(Value::Null), e.span()))),
                        ),
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
            .map_with(|(((key, value), head), body), e| {
                (
                    Expr::Sum(Box::new(Sum {
                        key,
                        value,
                        head,
                        body,
                    })),
                    e.span(),
                )
            });

        inline_expr.or(if_).or(sum)
    })
}
