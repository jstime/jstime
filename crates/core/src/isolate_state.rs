use rusty_v8 as v8;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct IsolateState {
    pub(crate) context: v8::Global<v8::Context>,
    pub(crate) module_map: crate::module::ModuleMap,
}

impl IsolateState {
    pub(crate) fn new(context: v8::Global<v8::Context>) -> Rc<RefCell<IsolateState>> {
        Rc::new(RefCell::new(IsolateState {
            context,
            module_map: crate::module::ModuleMap::new(),
        }))
    }

    pub(crate) fn get(scope: &mut v8::Isolate) -> Rc<RefCell<Self>> {
        scope
            .get_slot::<Rc<RefCell<IsolateState>>>()
            .unwrap()
            .clone()
    }
}
