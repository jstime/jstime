use std::cell::RefCell;
use std::rc::Rc;

/// Macro to get or create a cached V8 string key
/// Usage: get_cached_string!(cache, scope, field_name, "string_literal")
#[macro_export]
macro_rules! get_cached_string {
    ($cache:expr, $scope:expr, $field:ident, $str:expr) => {
        if let Some(ref cached) = $cache.$field {
            v8::Local::new($scope, cached)
        } else {
            let key = v8::String::new($scope, $str).unwrap();
            $cache.$field = Some(v8::Global::new($scope, key));
            key
        }
    };
}

pub(crate) struct FetchRequest {
    pub(crate) url: String,
    pub(crate) method: String,
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) body: Option<String>,
    pub(crate) resolver: v8::Global<v8::PromiseResolver>,
}

pub(crate) struct StringCache {
    // Fetch API keys
    pub(crate) body: Option<v8::Global<v8::String>>,
    pub(crate) status: Option<v8::Global<v8::String>>,
    pub(crate) status_text: Option<v8::Global<v8::String>>,
    pub(crate) headers: Option<v8::Global<v8::String>>,
    
    // fs API stat keys
    pub(crate) is_file: Option<v8::Global<v8::String>>,
    pub(crate) is_directory: Option<v8::Global<v8::String>>,
    pub(crate) is_symbolic_link: Option<v8::Global<v8::String>>,
    pub(crate) size: Option<v8::Global<v8::String>>,
    pub(crate) mtime_ms: Option<v8::Global<v8::String>>,
    
    // fs API options keys
    pub(crate) recursive: Option<v8::Global<v8::String>>,
    
    // Common error keys
    pub(crate) stack: Option<v8::Global<v8::String>>,
}

impl StringCache {
    pub(crate) fn new() -> Self {
        Self {
            // Fetch API keys
            body: None,
            status: None,
            status_text: None,
            headers: None,
            
            // fs API stat keys
            is_file: None,
            is_directory: None,
            is_symbolic_link: None,
            size: None,
            mtime_ms: None,
            
            // fs API options keys
            recursive: None,
            
            // Common error keys
            stack: None,
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
