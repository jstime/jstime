use smallvec::SmallVec;

// WHATWG Standards
mod whatwg {
    pub(crate) mod base64_impl;
    pub(crate) mod console_impl;
    pub(crate) mod event_impl;
    pub(crate) mod fetch_impl;
    pub(crate) mod queue_microtask_impl;
    pub(crate) mod streams_impl;
    pub(crate) mod structured_clone_impl;
    pub(crate) mod text_encoding_impl;
    pub(crate) mod timers_impl;
    pub(crate) mod url_impl;
}

// W3C Standards
mod w3c {
    pub(crate) mod crypto_impl;
    pub(crate) mod performance_impl;
}

// Node.js Compatible APIs
mod node {
    pub(crate) mod fs_impl;
    pub(crate) mod process_impl;
}

pub(crate) fn get_external_references() -> SmallVec<[v8::ExternalReference; 128]> {
    // Pre-allocate with capacity to avoid reallocation
    // Total: 2 (base64) + 1 (console) + 6 (event) + 1 (queue_microtask) + 13 (url) + 3 (timers)
    //        + 1 (fetch) + 3 (streams) + 1 (structured_clone) + 3 (text_encoding)
    //        + 3 (crypto) + 2 (performance) + 21 (fs) + 7 (process) = 67
    let mut refs = SmallVec::new();

    // WHATWG
    refs.extend(whatwg::base64_impl::get_external_references());
    refs.extend(whatwg::console_impl::get_external_references());
    refs.extend(whatwg::event_impl::get_external_references());
    refs.extend(whatwg::queue_microtask_impl::get_external_references());
    refs.extend(whatwg::url_impl::get_external_references());
    refs.extend(whatwg::timers_impl::get_external_references());
    refs.extend(whatwg::fetch_impl::get_external_references());
    refs.extend(whatwg::streams_impl::get_external_references());
    refs.extend(whatwg::structured_clone_impl::get_external_references());
    refs.extend(whatwg::text_encoding_impl::get_external_references());

    // W3C
    refs.extend(w3c::crypto_impl::get_external_references());
    refs.extend(w3c::performance_impl::get_external_references());

    // Node.js
    refs.extend(node::fs_impl::get_external_references());
    refs.extend(node::process_impl::get_external_references());

    refs
}

pub(crate) struct Builtins {}

impl Builtins {
    pub(crate) fn create(scope: &mut v8::PinScope) {
        let bindings = v8::Object::new(scope);

        // Register all builtin bindings
        // WHATWG
        whatwg::base64_impl::register_bindings(scope, bindings);
        whatwg::console_impl::register_bindings(scope, bindings);
        whatwg::event_impl::register_bindings(scope, bindings);
        whatwg::queue_microtask_impl::register_bindings(scope, bindings);
        whatwg::url_impl::register_bindings(scope, bindings);
        whatwg::timers_impl::register_bindings(scope, bindings);
        whatwg::fetch_impl::register_bindings(scope, bindings);
        whatwg::streams_impl::register_bindings(scope, bindings);
        whatwg::structured_clone_impl::register_bindings(scope, bindings);
        whatwg::text_encoding_impl::register_bindings(scope, bindings);

        // W3C
        w3c::crypto_impl::register_bindings(scope, bindings);
        w3c::performance_impl::register_bindings(scope, bindings);

        // Node.js
        node::fs_impl::register_bindings(scope, bindings);
        node::process_impl::register_bindings(scope, bindings);

        macro_rules! builtin {
            ($name:expr) => {
                let source = include_str!($name);
                let val = match crate::script::run(scope, source, $name) {
                    Ok(v) => v,
                    Err(_) => unreachable!(),
                };
                let func = v8::Local::<v8::Function>::try_from(val).unwrap();
                let recv = v8::undefined(scope).into();
                let args = [bindings.into()];
                func.call(scope, recv, &args).unwrap();
            };
        }

        // WHATWG
        builtin!("./whatwg/base64.js");
        builtin!("./whatwg/console.js");
        builtin!("./whatwg/event.js");
        builtin!("./whatwg/queue_microtask.js");
        builtin!("./whatwg/url.js");
        builtin!("./whatwg/timers.js");
        builtin!("./whatwg/fetch.js");
        builtin!("./whatwg/streams.js");
        builtin!("./whatwg/structured_clone.js");
        builtin!("./whatwg/text_encoding.js");

        // W3C
        builtin!("./w3c/crypto.js");
        builtin!("./w3c/performance.js");

        // Node.js
        builtin!("./node/fs.js");
        builtin!("./node/process.js");

        // Polyfills (load last to override native implementations)
        // Date locale methods polyfill - replaces broken V8 ICU implementation
        // TODO: Remove this polyfill when V8's native ICU implementation works correctly
        // with proper ICU data. Track V8 updates and test if toLocaleString() works
        // without this polyfill in future versions.
        let date_locale_polyfill = include_str!("./polyfills/date_locale.js");
        crate::script::run(scope, date_locale_polyfill, "date_locale_polyfill.js").unwrap();
    }
}
