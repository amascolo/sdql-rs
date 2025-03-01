use derive_more::Display;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type<'src> {
    Bool,
    Date,
    Int,
    Long,
    Real,
    String {
        max_len: Option<i32>,
    },
    Record(Vec<RecordType<'src>>),
    Dict {
        key: Box<Self>,
        val: Box<Self>,
        hint: Option<DictHint>,
    },
}

#[derive(Clone, Debug, Display, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[display("{name}: {type}")]
pub struct RecordType<'src> {
    pub name: Field<'src>,
    pub r#type: Type<'src>,
}
impl<'src> RecordType<'src> {
    pub fn concat(
        mut lhs: Vec<RecordType<'src>>,
        rhs: Vec<RecordType<'src>>,
    ) -> Vec<RecordType<'src>> {
        let fs1: HashMap<Field, Type> = lhs
            .iter()
            .map(|RecordType { name, r#type }| (name.clone(), r#type.clone()))
            .collect();
        let mut fs2: HashMap<Field, Type> = rhs
            .iter()
            .map(|RecordType { name, r#type }| (name.clone(), r#type.clone()))
            .collect();

        let common: HashMap<_, _> = fs1
            .into_iter()
            .filter_map(|(field, r#type)| {
                fs2.remove(&field)
                    .map(|type_rhs| (field, (r#type, type_rhs)))
            })
            .collect();
        drop(fs2);

        common.iter().for_each(|(name, (t1, t2))| {
            assert_eq!(t1, t2, "concat with different types for the field '{name}'")
        });

        lhs.extend(
            rhs.into_iter()
                .filter(|RecordType { name, .. }| !common.contains_key(name)),
        );
        lhs
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DictHint {
    HashDict,
    SortDict,
    SmallVecDict,
    Vec,
}

#[derive(Clone, Debug, Display, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[display("{_0}")]
pub struct Field<'src>(&'src str);

impl<'src> From<&'src str> for Field<'src> {
    fn from(s: &'src str) -> Self {
        Field(s)
    }
}

impl<'src> Type<'src> {
    #[allow(non_snake_case)]
    pub fn Set(r#type: Type<'src>) -> Self {
        Type::Dict {
            key: Box::new(r#type),
            val: Box::new(Type::Bool),
            hint: None,
        }
    }
}

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Date => write!(f, "date"),
            Self::Int => write!(f, "int"),
            Self::Long => write!(f, "long"),
            Self::Real => write!(f, "real"),
            Self::String {
                max_len: Some(max_len),
            } => write!(f, "string({max_len})"),
            Self::String { max_len: None } => write!(f, "string"),
            Self::Record(fields) => {
                write!(
                    f,
                    "<{}>",
                    fields
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::Dict { key, val, hint } => match hint {
                Some(hint) => write!(f, "@{hint} {{{key} -> {val}}}"),
                None => write!(f, "{{{key} -> {val}}}"),
            },
        }
    }
}

impl fmt::Display for DictHint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HashDict => "hashdict",
            Self::SortDict => "sortdict",
            Self::SmallVecDict => "smallvecdict",
            Self::Vec => "vec",
        }
        .fmt(f)
    }
}
