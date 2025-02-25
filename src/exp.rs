// use crate::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Exp {
    Sym(String),
    Const(Value),
    RecNode(Vec<(Field, Box<Exp>)>),
    DictNode(DictNode),
    RangeNode(Box<Exp>),
    Add(Box<Exp>, Box<Exp>),
    Mult(Box<Exp>, Box<Exp>),
    Neg(Box<Exp>),
    Cmp(Box<Exp>, Box<Exp>, String),
    IfThenElse(Box<Exp>, Box<Exp>, Box<Exp>),
    Sum(Sym, Sym, Box<Exp>, Box<Exp>),
    Get(Box<Exp>, Box<Exp>),
    // LetBinding(Sym, Box<Exp>, Box<Exp>),
    // Load(String, Type, DictNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Bool(bool),
    Int(i32),
    Long(i64),
    Float(f64),
}

pub type Field = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Sym(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DictNode(Vec<(Box<Exp>, Box<Exp>)>);
