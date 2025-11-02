pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(set_timeout),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(set_interval),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(clear_timer),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "setTimeout").unwrap();
    let value = v8::Function::new(scope, set_timeout).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "setInterval").unwrap();
    let value = v8::Function::new(scope, set_interval).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "clearTimer").unwrap();
    let value = v8::Function::new(scope, clear_timer).unwrap();
    bindings.set(scope, name.into(), value.into());
}

#[inline]
fn set_timeout(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let callback_obj = args.get(0);
    let callback = match crate::error::try_get_function_result(callback_obj) {
        Ok(f) => f,
        Err(msg) => {
            crate::error::throw_type_error(scope, msg);
            return;
        }
    };

    let delay_obj = args.get(1);
    let delay_ms = if delay_obj.is_undefined() || delay_obj.is_null() {
        0
    } else {
        delay_obj.integer_value(scope).unwrap_or(0).max(0) as u64
    };

    let callback_global = v8::Global::new(scope, callback);
    let state = crate::IsolateState::get(scope);
    let state_borrow = state.borrow();

    // Get the next timer ID
    let timer_id = {
        let mut next_id = state_borrow.next_timer_id.borrow_mut();
        let id = crate::event_loop::TimerId(*next_id);
        *next_id += 1;
        id
    };

    // Queue the timer to be added
    state_borrow
        .timers_to_add
        .borrow_mut()
        .push(crate::event_loop::PendingTimer::Timeout {
            id: timer_id,
            callback: callback_global,
            delay_ms,
        });
    // Return the timer ID as a number
    let timer_id_value = v8::Number::new(scope, timer_id.0 as f64);
    rv.set(timer_id_value.into());
}

#[inline]
fn set_interval(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let callback_obj = args.get(0);
    let callback = match crate::error::try_get_function_result(callback_obj) {
        Ok(f) => f,
        Err(msg) => {
            crate::error::throw_type_error(scope, msg);
            return;
        }
    };

    let interval_obj = args.get(1);
    let interval_ms = if interval_obj.is_undefined() || interval_obj.is_null() {
        0
    } else {
        interval_obj.integer_value(scope).unwrap_or(0).max(0) as u64
    };

    let callback_global = v8::Global::new(scope, callback);
    let state = crate::IsolateState::get(scope);
    let state_borrow = state.borrow();

    // Get the next timer ID
    let timer_id = {
        let mut next_id = state_borrow.next_timer_id.borrow_mut();
        let id = crate::event_loop::TimerId(*next_id);
        *next_id += 1;
        id
    };

    // Queue the timer to be added
    state_borrow
        .timers_to_add
        .borrow_mut()
        .push(crate::event_loop::PendingTimer::Interval {
            id: timer_id,
            callback: callback_global,
            interval_ms,
        });

    // Return the timer ID as a number
    let timer_id_value = v8::Number::new(scope, timer_id.0 as f64);
    rv.set(timer_id_value.into());
}

#[inline]
fn clear_timer(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let timer_id_obj = args.get(0);
    if let Some(timer_id_num) = timer_id_obj.number_value(scope) {
        let timer_id = crate::event_loop::TimerId(timer_id_num as u64);
        let state = crate::IsolateState::get(scope);
        state.borrow().timers_to_clear.borrow_mut().push(timer_id);
    }
}
