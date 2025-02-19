use chumsky::prelude::*;
use std::fmt;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Token<'src> {
    Null,
    Bool(bool),
    Num(f64),
    Str(&'src str),
    Op(&'src str),
    Ctrl(char),
    Ident(&'src str),
    Let,
    If,
    Then,
    Else,
    In,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Null => write!(f, "null"),
            Token::Bool(x) => write!(f, "{}", x),
            Token::Num(n) => write!(f, "{}", n),
            Token::Str(s) => write!(f, "{}", s),
            Token::Op(s) => write!(f, "{}", s),
            Token::Ctrl(c) => write!(f, "{}", c),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Let => write!(f, "let"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::In => write!(f, "in"),
        }
    }
}

pub(super) fn lexer<'src>()
-> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, extra::Err<Rich<'src, char, Span>>> {
    let num = text::int(10)
        .then(just('.').then(text::digits(10)).or_not())
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Num);

    let str_ = just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(Token::Str);

    let op = just("==")
        .or(just("!="))
        .or(just("!"))
        .or(just("-"))
        .or(just("="))
        .or(just("+"))
        .or(just("*"))
        .or(just("/"))
        .repeated()
        .exactly(1)
        .to_slice()
        .map(Token::Op);

    let ctrl = one_of("()[]{};,").map(Token::Ctrl);

    let ident = text::ascii::ident().map(|ident: &str| match ident {
        "let" => Token::Let,
        "in" => Token::In,
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "null" => Token::Null,
        _ => Token::Ident(ident),
    });

    let token = num.or(str_).or(op).or(ctrl).or(ident);

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
