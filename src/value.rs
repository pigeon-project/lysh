
use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use std::fmt::Debug;
// use std::fmt::Display;

pub type Ref<T> = Arc<T>;


pub trait LyshObjectShow: std::fmt::Debug {
    fn format(&self) -> String {
        format!("<object {:?}>", self)
    }
    fn mem_image(&self) -> Vec<u8> {
        Vec::new()
    }
}


#[derive(Debug, Clone)]
pub struct Rational (pub i64, pub i64);


#[derive(Debug, Clone)]
pub struct LPair (pub LyshValue, pub LyshValue);
/* {
    pub car: LyshValue,
    pub cdr: LyshValue,
} */


#[derive(Debug, Clone)]
pub struct LStruct {
    pub struct_info: (),
    pub item: Vec<LyshValue>,
}

pub struct ExecError {
    pub pos: Vec<u64>,
}

pub type ExecResult = Result<LyshValue, ExecError>;

// LyshNativeInterface
pub type LNI = Ref<dyn Fn(LyshValue, &Vec<LyshValue>) -> ExecResult>;

#[derive(Debug, Clone)]
pub struct FunMataInfo {
    pub name: Ref<String>,
    pub target: Ref<String>,
    pub exec_type: Ref<String>,
}

#[derive(Clone)]
pub struct LFunction {
    pub mata: FunMataInfo,
    pub body: LyshValue,
    pub exec: LNI,
}

#[derive(Clone)]
pub struct LClosure {
    pub fun: LFunction,
    pub env: LStruct,
}

impl Debug for LFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Function {} '{}'>", self.mata.exec_type, self.mata.name)
    }
}

impl Debug for LClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Closure {} '{}'>", self.fun.mata.exec_type, self.fun.mata.name)
    }
}

#[derive(Debug, Clone)]
pub enum LyshValue {
    Nil,
    Bool    (bool),
    Char    (char),
    Uint    (u64),
    Integer (i64),
    Float   (f64),
    Rational(Ref<Rational>),
    RString (Ref<String>),
    Symbol  (Ref<String>),
    Array   (Ref<Vec<LyshValue>>),
    Tuple   (Ref<Vec<LyshValue>>),
    List    (Ref<LPair>),
    Dict    (Ref<(HashMap<String, LyshValue>)>),
    Struct  (Ref<LStruct>),
    Function(Ref<LFunction>),
    Closure (Ref<LClosure>),
    Lock    (Ref<RwLock<LyshValue>>),
    Other   (Ref<Arc<dyn LyshObjectShow>>),
}

pub struct RowValueImage(pub u64, pub u64);

impl LyshValue {
    pub fn is_atom(&self) -> bool {
        match self {
            LyshValue::Nil          |
            LyshValue::Bool     (_) |
            LyshValue::Char     (_) |
            LyshValue::Uint     (_) |
            LyshValue::Integer  (_) |
            // LyshValue::Rational (_) |
            LyshValue::Float    (_) => true,
            _ => false,
        }
    }

    pub fn is_mutable(&self) -> bool {
        match self {
            LyshValue::Lock (_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            LyshValue::Nil => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            LyshValue::List (_) => true,
            _ => false,
        }
    }
}

impl LyshObjectShow for LyshValue {
    fn format(&self) -> String {
        match self {
            LyshValue::Nil => format!("nil"),
            LyshValue::Bool(v) => format!("{}", v),
            LyshValue::Char(v) => format!("'{}'", v),
            LyshValue::Uint(v) => format!("{}", v),
            LyshValue::Integer(v) => format!("{}", v),
            LyshValue::Rational(v) => format!("{}/{}", v.0, v.1),
            LyshValue::Float(v) => format!("{}", v),
            LyshValue::RString(v) => format!("{}", v),
            LyshValue::Symbol(v) => format!("`{}", v),
            LyshValue::Array(v) => format!("{:?}", *v),
            LyshValue::Dict(v) => format!("<dict {:?}>", &v), // FIXME
            LyshValue::Lock(v) => {
                let r = v.read().unwrap();
                format!("(lock {:?})", *r)
            }
            _ => format!("<object {:?}>", &self),
        }
    }
}
