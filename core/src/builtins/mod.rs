// Macro to define builtins and generate all the boilerplate code
macro_rules! define_builtins {
    (
        $(
            $category:ident: [
                $( ($impl_name:ident, $js_name:literal) ),* $(,)?
            ]
        ),* $(,)?
    ) => {
        // Generate module declarations
        $(
            mod $category {
                $(
                    pub(crate) mod $impl_name;
                )*
            }
        )*

        pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
            // Pre-allocate with approximate capacity to avoid reallocation
            let mut refs = Vec::with_capacity(50);

            $(
                $(
                    refs.extend($category::$impl_name::get_external_references());
                )*
            )*

            refs
        }

        pub(crate) struct Builtins {}

        impl Builtins {
            pub(crate) fn create(scope: &mut v8::PinScope) {
                let bindings = v8::Object::new(scope);

                // Register all builtin bindings
                $(
                    $(
                        $category::$impl_name::register_bindings(scope, bindings);
                    )*
                )*

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

                // Load JavaScript builtin files
                $(
                    $(
                        builtin!(concat!("./", stringify!($category), "/", $js_name, ".js"));
                    )*
                )*
            }
        }
    };
}

// Define all builtins here - each category with its implementations
define_builtins! {
    // WHATWG Standards
    whatwg: [
        (base64_impl, "base64"),
        (console_impl, "console"),
        (event_impl, "event"),
        (fetch_impl, "fetch"),
        (queue_microtask_impl, "queue_microtask"),
        (structured_clone_impl, "structured_clone"),
        (text_encoding_impl, "text_encoding"),
        (timers_impl, "timers"),
        (url_impl, "url"),
    ],
    // W3C Standards
    w3c: [
        (performance_impl, "performance"),
    ],
    // Node.js Compatible APIs
    node: [
        (fs_impl, "fs"),
    ],
}
