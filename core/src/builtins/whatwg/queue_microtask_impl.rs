pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![v8::ExternalReference {
        function: v8::MapFnTo::map_fn_to(queue_microtask),
    }]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "queueMicrotask").unwrap();
    let value = v8::Function::new(scope, queue_microtask).unwrap();
    bindings.set(scope, name.into(), value.into());
}

fn queue_microtask(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let obj = args.get(0);
    let func = v8::Local::<v8::Function>::try_from(obj).unwrap();
    scope.enqueue_microtask(func);
}
