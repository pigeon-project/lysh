
use crate::value::Ref;
use crate::value::LNI;
use crate::value::LyshValue;
use crate::value::ExecResult;
use crate::exec_engine::context::ThreadContext;
use crate::exec_engine::context::ExecEngine;

// Ast engine context info
#[derive(Debug, Clone)]
pub struct AstEngine {
    pub context: Ref<ThreadContext>,
}

impl AstEngine {
    fn new(context: Ref<ThreadContext>) -> Self {
        AstEngine {
            context
        }
    }

    fn eval_node(&self, node: LyshValue) -> ExecResult {
        match node {
            LyshValue::Nil |
            LyshValue::Bool (_) |
            LyshValue::Char (_) |
            LyshValue::Uint (_) |
            LyshValue::Float (_) |
            LyshValue::Integer  (_) |
            LyshValue::Rational (_) |
            LyshValue::RString (_) => Ok(node.clone()),
            _ => panic!("Unexecutable Ast"),
        }
    }
}

impl ExecEngine for AstEngine {
    fn new_engine(self) -> LNI {
        Ref::new(move |body, _args| {
            self.eval_node(body);
            unreachable!()
        })
    }

    fn new_debug_engine(self) -> LNI {
        self.new_engine()
    }
}
