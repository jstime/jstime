use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(performance_now),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(performance_time_origin),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "performanceNow").unwrap();
    let value = v8::Function::new(scope, performance_now).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "performanceTimeOrigin").unwrap();
    let value = v8::Function::new(scope, performance_time_origin).unwrap();
    bindings.set(scope, name.into(), value.into());
}

// Store the time origin when the isolate is created
thread_local! {
    static TIME_ORIGIN: std::cell::OnceCell<f64> = const { std::cell::OnceCell::new() };
}

fn performance_now(
    scope: &mut v8::PinScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let time_origin = TIME_ORIGIN.with(|cell| {
        *cell.get_or_init(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64()
                * 1000.0
        })
    });

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
        * 1000.0;

    let elapsed = now - time_origin;
    let value = v8::Number::new(scope, elapsed);
    rv.set(value.into());
}

fn performance_time_origin(
    scope: &mut v8::PinScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let time_origin = TIME_ORIGIN.with(|cell| {
        *cell.get_or_init(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64()
                * 1000.0
        })
    });

    let value = v8::Number::new(scope, time_origin);
    rv.set(value.into());
}
