pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(event_target_add_event_listener),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(event_target_remove_event_listener),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(event_target_dispatch_event),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(event_stop_propagation),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(event_stop_immediate_propagation),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(event_prevent_default),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    // EventTarget methods
    let name = v8::String::new(scope, "eventTargetAddEventListener").unwrap();
    let value = v8::Function::new(scope, event_target_add_event_listener).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "eventTargetRemoveEventListener").unwrap();
    let value = v8::Function::new(scope, event_target_remove_event_listener).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "eventTargetDispatchEvent").unwrap();
    let value = v8::Function::new(scope, event_target_dispatch_event).unwrap();
    bindings.set(scope, name.into(), value.into());

    // Event methods
    let name = v8::String::new(scope, "eventStopPropagation").unwrap();
    let value = v8::Function::new(scope, event_stop_propagation).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "eventStopImmediatePropagation").unwrap();
    let value = v8::Function::new(scope, event_stop_immediate_propagation).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "eventPreventDefault").unwrap();
    let value = v8::Function::new(scope, event_prevent_default).unwrap();
    bindings.set(scope, name.into(), value.into());
}

// EventTarget.addEventListener(type, listener, options)
#[inline]
fn event_target_add_event_listener(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    // args[0] = target (EventTarget instance)
    // args[1] = type (event type string)
    // args[2] = listener (function)
    // args[3] = options (optional)

    if args.length() < 3 {
        return;
    }

    let target = args.get(0);
    let type_arg = args.get(1);
    let listener = args.get(2);

    // Validate inputs early - fast rejection path
    if !listener.is_function() {
        return;
    }

    let target_obj = match v8::Local::<v8::Object>::try_from(target) {
        Ok(obj) => obj,
        Err(_) => return,
    };

    let listener_func = v8::Local::<v8::Function>::try_from(listener).unwrap();

    // Convert type to a V8 string - fast path if already a string
    let type_key = if type_arg.is_string() {
        v8::Local::<v8::String>::try_from(type_arg).unwrap()
    } else {
        v8::tc_scope!(let tc, scope);
        match type_arg.to_string(tc) {
            Some(s) => s,
            None => return,
        }
    };

    // Get cached __listeners__ key string from the isolate state
    // Acquire cache once and release quickly to minimize lock contention
    let listeners_key = {
        let isolate_state = crate::isolate_state::IsolateState::get(scope);
        let string_cache_ref = isolate_state.borrow().string_cache.clone();
        let mut cache = string_cache_ref.borrow_mut();
        crate::get_or_create_cached_string!(scope, cache, listeners, "__listeners__")
    };

    // Get the listeners map - should always exist since EventTarget constructor creates it
    let listeners_map = match target_obj.get(scope, listeners_key.into()) {
        Some(val) if val.is_map() => v8::Local::<v8::Map>::try_from(val).unwrap(),
        _ => {
            // Fallback: create new map if it doesn't exist (shouldn't happen normally)
            let new_map = v8::Map::new(scope);
            target_obj.set(scope, listeners_key.into(), new_map.into());
            new_map
        }
    };

    // Get or create the array of listeners for this event type
    match listeners_map.get(scope, type_key.into()) {
        Some(arr) if arr.is_array() => {
            // Fast path: array exists, just append
            let array = v8::Local::<v8::Array>::try_from(arr).unwrap();
            let length = array.length();
            array.set_index(scope, length, listener_func.into());
        }
        _ => {
            // Create new array for this event type with room for growth
            let array = v8::Array::new(scope, 4);
            array.set_index(scope, 0, listener_func.into());
            listeners_map.set(scope, type_key.into(), array.into());
        }
    }
}

// EventTarget.removeEventListener(type, listener, options)
#[inline]
fn event_target_remove_event_listener(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() < 3 {
        return;
    }

    let target = args.get(0);
    let type_arg = args.get(1);
    let listener = args.get(2);

    // Validate inputs early
    if !listener.is_function() {
        return;
    }

    let target_obj = match v8::Local::<v8::Object>::try_from(target) {
        Ok(obj) => obj,
        Err(_) => return,
    };

    // Convert type to a V8 string - fast path if already a string
    let type_key = if type_arg.is_string() {
        v8::Local::<v8::String>::try_from(type_arg).unwrap()
    } else {
        v8::tc_scope!(let tc, scope);
        match type_arg.to_string(tc) {
            Some(s) => s,
            None => return,
        }
    };

    // Get cached __listeners__ key string from the isolate state
    // Acquire cache once and release quickly
    let listeners_key = {
        let isolate_state = crate::isolate_state::IsolateState::get(scope);
        let string_cache_ref = isolate_state.borrow().string_cache.clone();
        let mut cache = string_cache_ref.borrow_mut();
        crate::get_or_create_cached_string!(scope, cache, listeners, "__listeners__")
    };

    // Early return if no listeners exist
    let listeners_map = match target_obj.get(scope, listeners_key.into()) {
        Some(val) if val.is_map() => v8::Local::<v8::Map>::try_from(val).unwrap(),
        _ => return,
    };

    let listeners_array = match listeners_map.get(scope, type_key.into()) {
        Some(val) if val.is_array() => v8::Local::<v8::Array>::try_from(val).unwrap(),
        _ => return,
    };

    // Find and remove the listener
    let listener_func = v8::Local::<v8::Function>::try_from(listener).unwrap();
    let length = listeners_array.length();
    
    // Pre-allocate new array with appropriate size
    let new_array = v8::Array::new(scope, 0);
    let mut new_index = 0;
    let mut removed = false;

    for i in 0..length {
        if let Some(item) = listeners_array.get_index(scope, i)
            && item.is_function()
        {
            let item_func = v8::Local::<v8::Function>::try_from(item).unwrap();
            // Compare function references - skip only the first match
            if !removed && item_func.strict_equals(listener_func.into()) {
                removed = true;
                continue;
            }
            new_array.set_index(scope, new_index, item);
            new_index += 1;
        }
    }

    // Update the map with the new array
    listeners_map.set(scope, type_key.into(), new_array.into());
}

// EventTarget.dispatchEvent(event)
// NOTE: This function is no longer the primary dispatch path.
// Event dispatching now happens in JavaScript (event.js) for better performance.
// This function is kept for backwards compatibility and snapshot support.
#[inline]
fn event_target_dispatch_event(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 2 {
        rv.set(v8::Boolean::new(scope, true).into());
        return;
    }

    let target = args.get(0);
    let event = args.get(1);

    let event_obj = match v8::Local::<v8::Object>::try_from(event) {
        Ok(obj) => obj,
        Err(_) => {
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }
    };

    let target_obj = match v8::Local::<v8::Object>::try_from(target) {
        Ok(obj) => obj,
        Err(_) => {
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }
    };

    // Get all cached strings at once to minimize borrow overhead
    let isolate_state = crate::isolate_state::IsolateState::get(scope);
    let string_cache_ref = isolate_state.borrow().string_cache.clone();
    let mut cache = string_cache_ref.borrow_mut();

    let current_target_key =
        crate::get_or_create_cached_string!(scope, cache, current_target, "__currentTarget__");
    let target_key = crate::get_or_create_cached_string!(scope, cache, target, "__target__");
    let type_key = crate::get_or_create_cached_string!(scope, cache, type_, "type");
    let listeners_key =
        crate::get_or_create_cached_string!(scope, cache, listeners, "__listeners__");
    let stop_immediate_key = crate::get_or_create_cached_string!(
        scope,
        cache,
        stop_immediate_propagation,
        "__stopImmediatePropagation__"
    );
    let default_prevented_key = crate::get_or_create_cached_string!(
        scope,
        cache,
        default_prevented,
        "__defaultPrevented__"
    );

    // Release the cache borrow early
    drop(cache);

    // Set currentTarget on the event
    event_obj.set(scope, current_target_key.into(), target);

    // Set target on the event if not already set
    let existing_target = event_obj.get(scope, target_key.into());
    if existing_target.is_none() || existing_target.unwrap().is_null_or_undefined() {
        event_obj.set(scope, target_key.into(), target);
    }

    // Get event type - use cached type string if available to avoid allocations
    // in the common case where the same event is dispatched multiple times
    let type_str_key = v8::String::new_external_onebyte_static(scope, b"__typeStr__").unwrap();

    let type_key_lookup = if let Some(cached) = event_obj.get(scope, type_str_key.into()) {
        if cached.is_string() {
            v8::Local::<v8::String>::try_from(cached).unwrap()
        } else {
            // Fallback: get type from event and cache it
            let type_val = match event_obj.get(scope, type_key.into()) {
                Some(val) => val,
                None => {
                    rv.set(v8::Boolean::new(scope, true).into());
                    return;
                }
            };

            let type_v8_str_opt = {
                v8::tc_scope!(let tc, scope);
                type_val.to_string(tc)
            };

            let type_v8_str = match type_v8_str_opt {
                Some(s) => s,
                None => {
                    rv.set(v8::Boolean::new(scope, true).into());
                    return;
                }
            };

            // Cache the V8 string on the event object for future dispatches
            event_obj.set(scope, type_str_key.into(), type_v8_str.into());

            type_v8_str
        }
    } else {
        // First dispatch: get type from event and cache it
        let type_val = match event_obj.get(scope, type_key.into()) {
            Some(val) => val,
            None => {
                rv.set(v8::Boolean::new(scope, true).into());
                return;
            }
        };

        let type_v8_str_opt = {
            v8::tc_scope!(let tc, scope);
            type_val.to_string(tc)
        };

        let type_v8_str = match type_v8_str_opt {
            Some(s) => s,
            None => {
                rv.set(v8::Boolean::new(scope, true).into());
                return;
            }
        };

        // Cache the V8 string on the event object for future dispatches
        event_obj.set(scope, type_str_key.into(), type_v8_str.into());

        type_v8_str
    };

    // Get listeners for this event type
    let listeners_map = match target_obj.get(scope, listeners_key.into()) {
        Some(val) if val.is_map() => v8::Local::<v8::Map>::try_from(val).unwrap(),
        _ => {
            // No listeners registered, return early
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }
    };

    // Use the already-converted type_key_lookup (which is a v8::String)
    let listeners_array = match listeners_map.get(scope, type_key_lookup.into()) {
        Some(val) if val.is_array() => v8::Local::<v8::Array>::try_from(val).unwrap(),
        _ => {
            // No listeners for this event type, return early
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }
    };

    // Call each listener
    let length = listeners_array.length();
    let event_arg = [event];

    for i in 0..length {
        // Get listener at index i and call it if it's a function
        if let Some(item) = listeners_array.get_index(scope, i)
            && item.is_function()
        {
            let listener_func = v8::Local::<v8::Function>::try_from(item).unwrap();
            listener_func.call(scope, target, &event_arg);

            // Check if immediate propagation was stopped after the call
            if let Some(stop_val) = event_obj.get(scope, stop_immediate_key.into())
                && stop_val.is_true()
            {
                break;
            }
        }
    }

    // Return !defaultPrevented
    let default_prevented = event_obj
        .get(scope, default_prevented_key.into())
        .map(|v| v.is_true())
        .unwrap_or(false);

    rv.set(v8::Boolean::new(scope, !default_prevented).into());
}

// Event.stopPropagation()
fn event_stop_propagation(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        return;
    }

    let event = args.get(0);
    let event_obj = match v8::Local::<v8::Object>::try_from(event) {
        Ok(obj) => obj,
        Err(_) => return,
    };

    // Get cached strings from the isolate state
    let isolate_state = crate::isolate_state::IsolateState::get(scope);
    let string_cache_ref = isolate_state.borrow().string_cache.clone();
    let mut cache = string_cache_ref.borrow_mut();

    let key =
        crate::get_or_create_cached_string!(scope, cache, stop_propagation, "__stopPropagation__");
    let value = v8::Boolean::new(scope, true);
    event_obj.set(scope, key.into(), value.into());
}

// Event.stopImmediatePropagation()
#[inline]
fn event_stop_immediate_propagation(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        return;
    }

    let event = args.get(0);
    let event_obj = match v8::Local::<v8::Object>::try_from(event) {
        Ok(obj) => obj,
        Err(_) => return,
    };

    // Get cached strings from the isolate state
    let isolate_state = crate::isolate_state::IsolateState::get(scope);
    let string_cache_ref = isolate_state.borrow().string_cache.clone();
    let mut cache = string_cache_ref.borrow_mut();

    let key = crate::get_or_create_cached_string!(
        scope,
        cache,
        stop_immediate_propagation,
        "__stopImmediatePropagation__"
    );
    let value = v8::Boolean::new(scope, true);
    event_obj.set(scope, key.into(), value.into());

    // Also stop propagation
    let prop_key =
        crate::get_or_create_cached_string!(scope, cache, stop_propagation, "__stopPropagation__");
    event_obj.set(scope, prop_key.into(), value.into());
}

// Event.preventDefault()
fn event_prevent_default(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        return;
    }

    let event = args.get(0);
    let event_obj = match v8::Local::<v8::Object>::try_from(event) {
        Ok(obj) => obj,
        Err(_) => return,
    };

    // Get cached strings from the isolate state
    let isolate_state = crate::isolate_state::IsolateState::get(scope);
    let string_cache_ref = isolate_state.borrow().string_cache.clone();
    let mut cache = string_cache_ref.borrow_mut();

    // Only allow preventDefault if cancelable
    let cancelable_key =
        crate::get_or_create_cached_string!(scope, cache, cancelable, "cancelable");
    let is_cancelable = event_obj
        .get(scope, cancelable_key.into())
        .map(|v| v.is_true())
        .unwrap_or(false);

    if is_cancelable {
        let key = crate::get_or_create_cached_string!(
            scope,
            cache,
            default_prevented,
            "__defaultPrevented__"
        );
        let value = v8::Boolean::new(scope, true);
        event_obj.set(scope, key.into(), value.into());
    }
}
