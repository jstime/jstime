use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct IsolateState {
    pub(crate) context: Option<v8::Global<v8::Context>>,
    pub(crate) module_map: crate::module::ModuleMap,
    pub(crate) event_loop: Rc<RefCell<crate::event_loop::EventLoop>>,
    pub(crate) timers_to_clear: Rc<RefCell<Vec<crate::event_loop::TimerId>>>,
    pub(crate) timers_to_add: Rc<RefCell<Vec<crate::event_loop::PendingTimer>>>,
    pub(crate) next_timer_id: Rc<RefCell<u64>>,
}

impl IsolateState {
    pub(crate) fn new(context: v8::Global<v8::Context>) -> Rc<RefCell<IsolateState>> {
        let timers_to_clear = Rc::new(RefCell::new(Vec::new()));
        let timers_to_add = Rc::new(RefCell::new(Vec::new()));
        let next_timer_id = Rc::new(RefCell::new(1u64));
        Rc::new(RefCell::new(IsolateState {
            context: Some(context),
            module_map: crate::module::ModuleMap::new(),
            event_loop: Rc::new(RefCell::new(crate::event_loop::EventLoop::new(
                timers_to_clear.clone(),
                timers_to_add.clone(),
                next_timer_id.clone(),
            ))),
            timers_to_clear,
            timers_to_add,
            next_timer_id,
        }))
    }

    pub(crate) fn get(scope: &mut v8::Isolate) -> Rc<RefCell<Self>> {
        scope
            .get_slot::<Rc<RefCell<IsolateState>>>()
            .unwrap()
            .clone()
    }

    pub(crate) fn context(&self) -> v8::Global<v8::Context> {
        match &self.context {
            Some(c) => c.clone(),
            None => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    // pub(crate) fn drop_context(&mut self) {
    //     self.context.take();
    // }
}
