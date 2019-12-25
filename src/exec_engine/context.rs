

use std::sync::RwLock;
use std::sync::Weak;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::value::Ref;
use crate::value::LNI;
use crate::value::LyshValue;
use crate::value::LFunction;
// use crate::value::ExecResult;

#[derive(Debug)]
pub struct Frame {
    // pub father: Option<Rc<FrameStack>>,
    pub callee_info: Ref<LFunction>,
    pub local_variable_table: Vec<LyshValue>,
}

#[derive(Debug)]
pub struct ThreadContext {
    pub runtime_context: Weak<RuntimeEnvContext>,
    pub frame_stack: RefCell<Vec<Frame>>,
    pub stack_max_length: usize,
}

#[derive(Debug)]
pub struct ModuleContext {

}

#[derive(Debug)]
pub struct RuntimeEnvContext {
    pub intern_pool: RwLock<HashMap<String, Ref<String>>>,
    pub module_table: RwLock<HashMap<String, ModuleContext>>,
}

impl RuntimeEnvContext {
    fn new() -> Self {
        RuntimeEnvContext {
            intern_pool: RwLock::new(HashMap::new()),
            module_table: RwLock::new(HashMap::new()),
        }
    }

    fn intern_string(&self, key: &String) -> Ref<String> {
        self.intern_pool
        .read().unwrap()
        .get(key)
        .map_or_else(
            || {
                let rs = Ref::new(key.to_string());
                self.intern_pool.write().unwrap().insert(key.to_string(), rs.clone()).unwrap();
                rs
            },
            |v| v.clone())
    }
}


pub trait ExecEngine {
    fn new_engine(self) -> LNI;
    fn new_debug_engine(self) -> LNI;
}