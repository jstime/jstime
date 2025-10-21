mod console;
mod fetch_impl;
mod performance_impl;
mod queue_microtask_impl;
mod timers_impl;
mod url_impl;

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    let mut refs = Vec::new();
    refs.extend(console::get_external_references());
    refs.extend(queue_microtask_impl::get_external_references());
    refs.extend(url_impl::get_external_references());
    refs.extend(timers_impl::get_external_references());
    refs.extend(fetch_impl::get_external_references());
    refs.extend(performance_impl::get_external_references());
    refs
}

pub(crate) struct Builtins {}

impl Builtins {
    pub(crate) fn create(scope: &mut v8::PinScope) {
        let bindings = v8::Object::new(scope);

        // Register all builtin bindings
        console::register_bindings(scope, bindings);
        queue_microtask_impl::register_bindings(scope, bindings);
        url_impl::register_bindings(scope, bindings);
        timers_impl::register_bindings(scope, bindings);
        fetch_impl::register_bindings(scope, bindings);
        performance_impl::register_bindings(scope, bindings);

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

        builtin!("./console.js");
        builtin!("./queue_microtask.js");
        builtin!("./url.js");
        builtin!("./timers.js");
        builtin!("./fetch.js");
        builtin!("./performance.js");
    }
}
