use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct FetchRequest {
    pub(crate) url: String,
    pub(crate) method: String,
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) body: Option<String>,
    pub(crate) resolver: v8::Global<v8::PromiseResolver>,
}

/// Stores response body data for streaming
pub(crate) struct StreamingFetch {
    pub(crate) stream_id: u64,
    pub(crate) body_data: Vec<u8>,
    pub(crate) offset: usize,
}

pub(crate) struct StringCache {
    pub(crate) body: Option<v8::Global<v8::String>>,
    pub(crate) status: Option<v8::Global<v8::String>>,
    pub(crate) status_text: Option<v8::Global<v8::String>>,
    pub(crate) headers: Option<v8::Global<v8::String>>,
}

impl StringCache {
    pub(crate) fn new() -> Self {
        Self {
            body: None,
            status: None,
            status_text: None,
            headers: None,
        }
    }
}

pub(crate) struct IsolateState {
    pub(crate) context: Option<v8::Global<v8::Context>>,
    pub(crate) module_map: crate::module::ModuleMap,
    pub(crate) event_loop: Rc<RefCell<crate::event_loop::EventLoop>>,
    pub(crate) timers_to_clear: Rc<RefCell<Vec<crate::event_loop::TimerId>>>,
    pub(crate) timers_to_add: Rc<RefCell<Vec<crate::event_loop::PendingTimer>>>,
    pub(crate) next_timer_id: Rc<RefCell<u64>>,
    pub(crate) pending_fetches: Rc<RefCell<Vec<FetchRequest>>>,
    pub(crate) string_cache: Rc<RefCell<StringCache>>,
    pub(crate) http_agent: ureq::Agent,
    pub(crate) process_argv: Vec<String>,
    pub(crate) next_stream_id: Rc<RefCell<u64>>,
    pub(crate) streaming_fetches: Rc<RefCell<rustc_hash::FxHashMap<u64, StreamingFetch>>>,
}

impl IsolateState {
    pub(crate) fn new(
        context: v8::Global<v8::Context>,
        process_argv: Vec<String>,
    ) -> Rc<RefCell<IsolateState>> {
        let timers_to_clear = Rc::new(RefCell::new(Vec::new()));
        let timers_to_add = Rc::new(RefCell::new(Vec::new()));
        let next_timer_id = Rc::new(RefCell::new(1u64));
        let pending_fetches = Rc::new(RefCell::new(Vec::new()));
        let string_cache = Rc::new(RefCell::new(StringCache::new()));
        let next_stream_id = Rc::new(RefCell::new(1u64));
        let streaming_fetches = Rc::new(RefCell::new(rustc_hash::FxHashMap::default()));

        // Create an HTTP agent for connection pooling
        // Configure to not treat HTTP status codes as errors (fetch API expects this)
        let config = ureq::config::Config::builder()
            .http_status_as_error(false)
            .build();
        let http_agent = ureq::Agent::new_with_config(config);

        Rc::new(RefCell::new(IsolateState {
            context: Some(context),
            module_map: crate::module::ModuleMap::new(),
            event_loop: Rc::new(RefCell::new(crate::event_loop::EventLoop::new(
                timers_to_clear.clone(),
                timers_to_add.clone(),
                next_timer_id.clone(),
                pending_fetches.clone(),
            ))),
            timers_to_clear,
            timers_to_add,
            next_timer_id,
            pending_fetches,
            string_cache,
            http_agent,
            process_argv,
            next_stream_id,
            streaming_fetches,
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

    pub(crate) fn drop_context(&mut self) {
        self.context.take();
    }
}
