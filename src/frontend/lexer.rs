use crate::ir::r#type::DictHint;
use chumsky::prelude::*;
use derive_more::Display;
use std::fmt;

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{_0}")]
pub struct Spanned<T>(pub T, pub SimpleSpan);
impl<T> Spanned<T> {
    pub fn map<U, F>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned(f(self.0), self.1)
    }
    pub fn boxed(self) -> Spanned<Box<T>> {
        self.map(Box::new)
    }
    pub fn unboxed<U>(self) -> Spanned<U>
    where
        T: Into<Box<U>>,
    {
        self.map(|boxed| *boxed.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'src> {
    Bool(bool),
    Integer(i64),
    Real(f64),
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
    Range,
    At,
    DictHint(DictHint),
    Dom,
    Load,
    Type(ScalarType),
    Concat,
    External,
    Promote,
    Unique,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScalarType {
    Bool,
    Date,
    Int,
    Long,
    Real,
    String,
    VarChar,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Bool(x) => write!(f, "{x}"),
            Token::Integer(n) => write!(f, "{n}"),
            Token::Real(x) => write!(f, "{x}"),
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
            Token::Range => write!(f, "range"),
            Token::At => write!(f, "@"),
            Token::DictHint(DictHint::HashDict) => write!(f, "hashdict"),
            Token::DictHint(DictHint::SortDict) => write!(f, "sortdict"),
            Token::DictHint(DictHint::SmallVecDict) => write!(f, "smallvecdict"),
            Token::DictHint(DictHint::Vec) => write!(f, "vec"),
            Token::Dom => write!(f, "dom"),
            Token::Load => write!(f, "load"),
            Token::Type(t) => write!(f, "{t}"),
            Token::Concat => write!(f, "concat"),
            Token::External => write!(f, "external"),
            Token::Promote => write!(f, "promote"),
            Token::Unique => write!(f, "unique"),
        }
    }
}

impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalarType::Bool => "bool",
            ScalarType::Date => "date",
            ScalarType::Int => "int",
            ScalarType::Long => "long",
            ScalarType::Real => "real",
            ScalarType::String => "string",
            ScalarType::VarChar => "varchar",
        }
        .fmt(f)
    }
}

pub(super) fn lexer<'src>()
-> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, extra::Err<Rich<'src, char, SimpleSpan>>>
{
    let real = text::int(10)
        .then(just('.').then(text::digits(10)))
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Real);

    let integer = text::int(10)
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Integer);

    let str_ = just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(Token::Str);

    let arrows = just("<-").or(just("->")).map(Token::Arrow);
    let at = just("@").map(|_| Token::At);

    let type_ = choice((
        just("bool").to(Token::Type(ScalarType::Bool)),
        just("date").to(Token::Type(ScalarType::Date)),
        just("int").to(Token::Type(ScalarType::Int)),
        just("long").to(Token::Type(ScalarType::Long)),
        just("real").to(Token::Type(ScalarType::Real)),
        just("string").to(Token::Type(ScalarType::String)),
        just("varchar").to(Token::Type(ScalarType::VarChar)),
    ));

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
        "range" => Token::Range,
        "hashdict" => Token::DictHint(DictHint::HashDict),
        "sortdict" => Token::DictHint(DictHint::SortDict),
        "smallvecdict" => Token::DictHint(DictHint::SmallVecDict),
        "dom" => Token::Dom,
        "vec" => Token::DictHint(DictHint::Vec),
        "load" => Token::Load,
        "concat" => Token::Concat,
        "external" => Token::External,
        "promote" => Token::Promote,
        "unique" => Token::Unique,
        _ => Token::Ident(ident),
    });

    let token = real
        .or(integer)
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
        .map_with(|tok, e| Spanned(tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
