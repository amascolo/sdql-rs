use chumsky::prelude::*;
use std::fmt;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span); // TODO newtype + display

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Token<'src> {
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
    Arrow(&'src str),
    Sum,
    At,
    DictHint(DictHint),
    Load,
    Type(ScalarType),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum DictHint {
    HashDict,
    SortDict,
    SmallVecDict,
    Vec,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum ScalarType {
    String,
    Bool,
    Int,
    Long,
}

impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalarType::String => "string",
            ScalarType::Bool => "bool",
            ScalarType::Int => "int",
            ScalarType::Long => "long",
        }
        .fmt(f)
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Bool(x) => write!(f, "{x}"),
            Token::Num(n) => write!(f, "{n}"),
            Token::Str(s) => write!(f, "\"{s}\""),
            Token::Op(s) => write!(f, "{s}"),
            Token::Ctrl(c) => write!(f, "{c}"),
            Token::Ident(s) => write!(f, "{s}"),
            Token::Let => write!(f, "let"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::In => write!(f, "in"),
            Token::Arrow(s) => write!(f, "{s}"),
            Token::Sum => write!(f, "sum"),
            Token::At => write!(f, "@"),
            Token::DictHint(DictHint::HashDict) => write!(f, "hashdict"),
            Token::DictHint(DictHint::SortDict) => write!(f, "sortdict"),
            Token::DictHint(DictHint::SmallVecDict) => write!(f, "smallvecdict"),
            Token::DictHint(DictHint::Vec) => write!(f, "vec"),
            Token::Load => write!(f, "load"),
            Token::Type(t) => write!(f, "{t}"),
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

    let arrows = just("<-").or(just("->")).map(Token::Arrow);
    let at = just("@").map(|_| Token::At);

    let type_ = just("string")
        .or(just("bool").or(just("int").or(just("long"))))
        .map(|s| match s {
            "string" => Token::Type(ScalarType::String),
            "bool" => Token::Type(ScalarType::Bool),
            "int" => Token::Type(ScalarType::Int),
            "long" => Token::Type(ScalarType::Long),
            _ => unreachable!(),
        });

    let op = just("==")
        .or(just("!="))
        .or(just("<="))
        .or(just(">="))
        .or(just("&&"))
        .or(just("||"))
        .or(just("!"))
        .or(just("<"))
        .or(just(">"))
        .or(just("-"))
        .or(just("="))
        .or(just("+"))
        .or(just("*"))
        .or(just("/"))
        .map(Token::Op);

    let ctrl = one_of("()[]{}:;,.").map(Token::Ctrl);

    let ident = text::ascii::ident().map(|ident: &str| match ident {
        "let" => Token::Let,
        "in" => Token::In,
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "sum" => Token::Sum,
        "hashdict" => Token::DictHint(DictHint::HashDict),
        "sortdict" => Token::DictHint(DictHint::SortDict),
        "smallvecdict" => Token::DictHint(DictHint::SmallVecDict),
        "vec" => Token::DictHint(DictHint::Vec),
        "load" => Token::Load,
        _ => Token::Ident(ident),
    });

    let token = num
        .or(str_)
        .or(arrows)
        .or(op)
        .or(at)
        .or(ctrl)
        .or(type_)
        .or(ident);

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
