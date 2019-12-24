
// use std::rc::Rc;
use std::sync::Arc;
use std::sync::Weak;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::value::Ref;
use crate::value::LFunction;
use crate::value::LyshValue;


pub struct Frame {
    // pub father: Option<Rc<FrameStack>>,
    pub callee_info: Arc<LFunction>,
    pub local_variable_table: Vec<LyshValue>,
}

pub struct ThreadContext {
    pub runtime_context: Weak<RuntimeEnvContext>,
    pub frame_stack: RefCell<Vec<Frame>>,
    pub stack_max_length: usize,
}

pub struct ModuleContext {

}

pub struct RuntimeEnvContext {
    pub intern_pool: HashSet<String, Ref<String>>,
    pub module_table: HashMap<String, ModuleContext>,
}