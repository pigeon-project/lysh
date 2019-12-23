
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

// pub struct Ref<T> (pub Arc<T>);

#[derive(Debug, Clone)]
pub struct LPair (pub LyshValue, pub LyshValue);
/* {
    pub car: LyshValue,
    pub cdr: LyshValue,
} */

#[derive(Debug, Clone)]
pub enum LyshValue {
    Nil,
    Bool    (bool),
    Char    (char),
    Int     (i64),
    Uint     (u64),
    Float   (f64),
    Symbol  (Rc<String>),
    RString (Rc<String>),
    Array   (Rc<Vec<LyshValue>>),
    List    (Rc<LPair>),
    Dict    (Rc<()>),
    Struct  (Rc<()>),
    Other   (Rc<()>),
    Lock    (Arc<RwLock<LyshValue>>),
}

impl LyshValue {
    pub fn isAtom(&self) -> bool {
        match self {
            LyshValue::Nil      |
            LyshValue::Bool (_) |
            LyshValue::Char (_) |
            LyshValue::Int  (_) |
            LyshValue::Uint (_) |
            LyshValue::Float (_) => true,
            _ => false,
        }
    }

    pub fn isImmutable(&self) -> bool {
        match self {
            LyshValue::Lock (_) => false,
            _ => true,
        }
    }

    pub fn isNil(&self) -> bool {
        match self {
            LyshValue::Nil => true,
            _ => false,
        }
    }

    pub fn isList(&self) -> bool {
        match self {
            LyshValue::List (_) => true,
            _ => false,
        }
    }
}

impl Display for LyshValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LyshValue::Nil => write!(f, "nil"),
            LyshValue::Bool(v) => write!(f, "{}", v),
            LyshValue::Char(v) => write!(f, "'{}'", v),
            LyshValue::Int(v) => write!(f, "{}", v),
            LyshValue::Uint(v) => write!(f, "{}", v),
            LyshValue::Float(v) => write!(f, "{}", v),
            LyshValue::RString(v) => write!(f, "{}", v),
            LyshValue::Symbol(v) => write!(f, "`{}", v),
            LyshValue::Array(v) => write!(f, "{:?}", *v),
            LyshValue::Dict(v) => write!(f, "<dict {:?}>", &v), // FIXME
            LyshValue::Lock(v) => {
                let r = v.read().unwrap();
                write!(f, "(lock {})", *r)
            }
            _ => write!(f, "<object {}>", &self),
        }
    }
}
