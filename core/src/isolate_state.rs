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
#[allow(dead_code)]
pub(crate) struct StreamingFetch {
    pub(crate) stream_id: u64,
    pub(crate) body_data: Vec<u8>,
    pub(crate) offset: usize,
}

/// Cache for frequently used V8 strings to avoid repeated UTF-8 ↔ V8 conversions.
/// This addresses a key performance bottleneck by caching commonly used property names
/// and string literals throughout the runtime.
///
/// Note: Some fields are not yet used but are included for comprehensive caching
/// opportunities across all builtins. The unused fields are intentionally kept
/// to support future optimizations.
#[allow(dead_code)]
pub(crate) struct StringCache {
    // Fetch-related strings
    pub(crate) status: Option<v8::Global<v8::String>>,
    pub(crate) status_text: Option<v8::Global<v8::String>>,
    pub(crate) headers: Option<v8::Global<v8::String>>,

    // Common property names
    pub(crate) name: Option<v8::Global<v8::String>>,
    pub(crate) type_: Option<v8::Global<v8::String>>,
    pub(crate) value: Option<v8::Global<v8::String>>,
    pub(crate) length: Option<v8::Global<v8::String>>,
    pub(crate) done: Option<v8::Global<v8::String>>,
    pub(crate) message: Option<v8::Global<v8::String>>,
    pub(crate) stack: Option<v8::Global<v8::String>>,

    // Crypto-related strings
    pub(crate) algorithm: Option<v8::Global<v8::String>>,
    pub(crate) hash: Option<v8::Global<v8::String>>,
    pub(crate) key_data: Option<v8::Global<v8::String>>,
    pub(crate) extractable: Option<v8::Global<v8::String>>,
    pub(crate) usages: Option<v8::Global<v8::String>>,
    pub(crate) secret: Option<v8::Global<v8::String>>,
    pub(crate) iv: Option<v8::Global<v8::String>>,
    pub(crate) additional_data: Option<v8::Global<v8::String>>,

    // Event-related strings
    pub(crate) listeners: Option<v8::Global<v8::String>>,
    pub(crate) stop_propagation: Option<v8::Global<v8::String>>,
    pub(crate) stop_immediate_propagation: Option<v8::Global<v8::String>>,
    pub(crate) default_prevented: Option<v8::Global<v8::String>>,
    pub(crate) current_target: Option<v8::Global<v8::String>>,
    pub(crate) target: Option<v8::Global<v8::String>>,
    pub(crate) cancelable: Option<v8::Global<v8::String>>,

    // File system strings
    pub(crate) is_file: Option<v8::Global<v8::String>>,
    pub(crate) is_directory: Option<v8::Global<v8::String>>,
    pub(crate) is_symbolic_link: Option<v8::Global<v8::String>>,
    pub(crate) size: Option<v8::Global<v8::String>>,
    pub(crate) mtime_ms: Option<v8::Global<v8::String>>,
    pub(crate) recursive: Option<v8::Global<v8::String>>,

    // Module/import strings
    pub(crate) url: Option<v8::Global<v8::String>>,

    // Process-related strings
    pub(crate) encoding: Option<v8::Global<v8::String>>,
    pub(crate) path: Option<v8::Global<v8::String>>,
    pub(crate) data: Option<v8::Global<v8::String>>,
}

impl StringCache {
    pub(crate) fn new() -> Self {
        Self {
            // Fetch-related
            status: None,
            status_text: None,
            headers: None,

            // Common property names
            name: None,
            type_: None,
            value: None,
            length: None,
            done: None,
            message: None,
            stack: None,

            // Crypto-related
            algorithm: None,
            hash: None,
            key_data: None,
            extractable: None,
            usages: None,
            secret: None,
            iv: None,
            additional_data: None,

            // Event-related
            listeners: None,
            stop_propagation: None,
            stop_immediate_propagation: None,
            default_prevented: None,
            current_target: None,
            target: None,
            cancelable: None,

            // File system
            is_file: None,
            is_directory: None,
            is_symbolic_link: None,
            size: None,
            mtime_ms: None,
            recursive: None,

            // Module/import
            url: None,

            // Process-related
            encoding: None,
            path: None,
            data: None,
        }
    }
}

/// Macro to get or create a cached string.
/// This simplifies the pattern of checking for cached strings and creating them if needed.
///
/// Usage: get_or_create_cached_string!(scope, cache, field_name, "literal")
#[macro_export]
macro_rules! get_or_create_cached_string {
    ($scope:expr, $cache:expr, $field:ident, $literal:expr) => {{
        if let Some(ref cached) = $cache.$field {
            v8::Local::new($scope, cached)
        } else {
            let key = v8::String::new($scope, $literal).unwrap();
            let isolate: &v8::Isolate = $scope;
            $cache.$field = Some(v8::Global::new(isolate, key));
            key
        }
    }};
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
    // Object pool for frequently allocated header vectors
    pub(crate) header_vec_pool: Rc<crate::pool::Pool<Vec<(String, String)>>>,
    // Buffered random number generator for crypto operations
    pub(crate) buffered_random: RefCell<crate::buffered_random::BufferedRandom>,
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

        // Create object pool for header vectors with reasonable capacity limit
        let header_vec_pool = Rc::new(crate::pool::Pool::new(200));

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
            header_vec_pool,
            buffered_random: RefCell::new(crate::buffered_random::BufferedRandom::new()),
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
