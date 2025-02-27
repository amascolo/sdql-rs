#![allow(dead_code)]

use crate::frontend::lexer::Spanned;
use crate::ir::expr::{BinaryOp, DictEntry, External, RecordValue, UnaryOp};
use crate::ir::r#type::{DictHint, Field, Type};
use time::Date;

#[derive(Clone, Debug, PartialEq)]
pub struct Typed<'src, T> {
    val: T,
    r#type: Type<'src>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypedExpr<'src> {
    Sym {
        val: &'src str,
    },
    Bool {
        val: bool,
    },
    Date {
        val: Date,
    },
    Float {
        val: f64,
    },
    Int {
        val: i32,
    },
    Long {
        val: i64,
    },
    String {
        val: &'src str,
    },
    Record {
        vals: Vec<RecordValue<'src>>,
    },
    Dict {
        map: Vec<DictEntry<'src>>,
        hint: Option<DictHint>,
    },
    Let {
        lhs: &'src str,
        rhs: Box<Typed<'src, Spanned<Self>>>,
        cont: Box<Typed<'src, Spanned<Self>>>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Typed<'src, Spanned<Self>>>,
    },
    Binary {
        lhs: Box<Typed<'src, Spanned<Self>>>,
        op: BinaryOp,
        rhs: Box<Typed<'src, Spanned<Self>>>,
    },
    If {
        r#if: Box<Typed<'src, Spanned<Self>>>,
        then: Box<Typed<'src, Spanned<Self>>>,
        r#else: Option<Box<Typed<'src, Spanned<Self>>>>,
    },
    Field {
        expr: Box<Typed<'src, Spanned<Self>>>,
        field: Field<'src>,
    },
    Get {
        lhs: Box<Typed<'src, Spanned<Self>>>,
        rhs: Box<Typed<'src, Spanned<Self>>>,
    },
    Load {
        r#type: Type<'src>,
        path: &'src str,
    },
    Sum {
        key: Box<Typed<'src, Spanned<Self>>>,
        val: Box<Typed<'src, Spanned<Self>>>,
        head: Box<Typed<'src, Spanned<Self>>>,
        body: Box<Typed<'src, Spanned<Self>>>,
    },
    Range {
        expr: Box<Typed<'src, Spanned<Self>>>,
    },
    Concat {
        lhs: Box<Typed<'src, Spanned<Self>>>,
        rhs: Box<Typed<'src, Spanned<Self>>>,
    },
    External {
        func: External,
        args: Vec<Typed<'src, Spanned<Self>>>,
    },
    Promote {
        promo: Type<'src>,
        expr: Box<Typed<'src, Spanned<Self>>>,
    },
    Unique {
        expr: Box<Typed<'src, Spanned<Self>>>,
    },
}

// TODO use im-rc for Ctx
