
use crate::value::LyshValue;

pub struct RowAst (pub LyshValue);

impl RowAst {
    pub fn new(i: &LyshValue) -> Option<Self> {
        if RowAst::check(i) {
            Some(RowAst(i.clone()))
        } else { None }
    }

    pub fn check(i: &LyshValue) -> bool {
        match i {
            LyshValue::Array    (_) |
            LyshValue::Dict     (_) |
            LyshValue::Struct   (_) |
            LyshValue::Other    (_) |
            LyshValue::Lock     (_) => false,
            LyshValue::List     (v) => {
                RowAst::check(&v.0) && ((&v.1).isList() || (&v.1).isNil())
            }
            _ => true
        }
    }
}

pub enum AstNode {

}