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

    // Validate inputs
    if !listener.is_function() {
        return;
    }

    let type_str = {
        v8::tc_scope!(let tc, scope);
        match type_arg.to_string(tc) {
            Some(s) => s.to_rust_string_lossy(tc),
            None => return,
        }
    };

    // Get or create the __listeners__ internal slot
    let listeners_key = v8::String::new(scope, "__listeners__").unwrap();
    let target_obj = match v8::Local::<v8::Object>::try_from(target) {
        Ok(obj) => obj,
        Err(_) => return,
    };

    let listeners_map = if let Some(existing) = target_obj.get(scope, listeners_key.into()) {
        if existing.is_map() {
            v8::Local::<v8::Map>::try_from(existing).unwrap()
        } else {
            v8::Map::new(scope)
        }
    } else {
        v8::Map::new(scope)
    };

    // Get the array of listeners for this event type
    let type_key = v8::String::new(scope, &type_str).unwrap();
    let listeners_array = if let Some(existing) = listeners_map.get(scope, type_key.into()) {
        if existing.is_array() {
            v8::Local::<v8::Array>::try_from(existing).unwrap()
        } else {
            v8::Array::new(scope, 0)
        }
    } else {
        v8::Array::new(scope, 0)
    };

    // Add the listener to the array
    let listener_func = v8::Local::<v8::Function>::try_from(listener).unwrap();
    let length = listeners_array.length();
    listeners_array.set_index(scope, length, listener_func.into());

    // Update the map
    listeners_map.set(scope, type_key.into(), listeners_array.into());

    // Store the map back on the target
    target_obj.set(scope, listeners_key.into(), listeners_map.into());
}

// EventTarget.removeEventListener(type, listener, options)
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

    if !listener.is_function() {
        return;
    }

    let type_str = {
        v8::tc_scope!(let tc, scope);
        match type_arg.to_string(tc) {
            Some(s) => s.to_rust_string_lossy(tc),
            None => return,
        }
    };

    let listeners_key = v8::String::new(scope, "__listeners__").unwrap();
    let target_obj = match v8::Local::<v8::Object>::try_from(target) {
        Ok(obj) => obj,
        Err(_) => return,
    };

    let listeners_map = match target_obj.get(scope, listeners_key.into()) {
        Some(val) if val.is_map() => v8::Local::<v8::Map>::try_from(val).unwrap(),
        _ => return,
    };

    let type_key = v8::String::new(scope, &type_str).unwrap();
    let listeners_array = match listeners_map.get(scope, type_key.into()) {
        Some(val) if val.is_array() => v8::Local::<v8::Array>::try_from(val).unwrap(),
        _ => return,
    };

    // Find and remove the listener
    let listener_func = v8::Local::<v8::Function>::try_from(listener).unwrap();
    let length = listeners_array.length();
    let new_array = v8::Array::new(scope, 0);
    let mut new_index = 0;

    for i in 0..length {
        if let Some(item) = listeners_array.get_index(scope, i)
            && item.is_function()
        {
            let item_func = v8::Local::<v8::Function>::try_from(item).unwrap();
            // Compare function references
            if !item_func.strict_equals(listener_func.into()) {
                new_array.set_index(scope, new_index, item);
                new_index += 1;
            }
        }
    }

    // Update the map with the new array
    listeners_map.set(scope, type_key.into(), new_array.into());
}

// EventTarget.dispatchEvent(event)
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

    // Set currentTarget on the event
    let current_target_key = v8::String::new(scope, "__currentTarget__").unwrap();
    event_obj.set(scope, current_target_key.into(), target);

    // Set target on the event if not already set
    let target_key = v8::String::new(scope, "__target__").unwrap();
    let existing_target = event_obj.get(scope, target_key.into());
    if existing_target.is_none() || existing_target.unwrap().is_null_or_undefined() {
        event_obj.set(scope, target_key.into(), target);
    }

    // Get event type
    let type_key = v8::String::new(scope, "type").unwrap();
    let type_val = match event_obj.get(scope, type_key.into()) {
        Some(val) => val,
        None => {
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }
    };

    let type_str = {
        v8::tc_scope!(let tc, scope);
        type_val.to_string(tc).map(|s| s.to_rust_string_lossy(tc))
    };

    let type_str = match type_str {
        Some(s) => s,
        None => {
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }
    };

    // Get listeners for this event type
    let listeners_key = v8::String::new(scope, "__listeners__").unwrap();
    let target_obj = match v8::Local::<v8::Object>::try_from(target) {
        Ok(obj) => obj,
        Err(_) => {
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }
    };

    let listeners_map = match target_obj.get(scope, listeners_key.into()) {
        Some(val) if val.is_map() => v8::Local::<v8::Map>::try_from(val).unwrap(),
        _ => {
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }
    };

    let type_key = v8::String::new(scope, &type_str).unwrap();
    let listeners_array = match listeners_map.get(scope, type_key.into()) {
        Some(val) if val.is_array() => v8::Local::<v8::Array>::try_from(val).unwrap(),
        _ => {
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }
    };

    // Check if propagation was stopped immediately
    let stop_immediate_key = v8::String::new(scope, "__stopImmediatePropagation__").unwrap();

    // Call each listener
    let length = listeners_array.length();
    for i in 0..length {
        // Check if immediate propagation was stopped
        if let Some(stop_val) = event_obj.get(scope, stop_immediate_key.into())
            && stop_val.is_true()
        {
            break;
        }

        if let Some(item) = listeners_array.get_index(scope, i)
            && item.is_function()
        {
            let listener_func = v8::Local::<v8::Function>::try_from(item).unwrap();
            let recv = target;
            let args = [event];
            listener_func.call(scope, recv, &args);
        }
    }

    // Return !defaultPrevented
    let default_prevented_key = v8::String::new(scope, "__defaultPrevented__").unwrap();
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

    let key = v8::String::new(scope, "__stopPropagation__").unwrap();
    let value = v8::Boolean::new(scope, true);
    event_obj.set(scope, key.into(), value.into());
}

// Event.stopImmediatePropagation()
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

    let key = v8::String::new(scope, "__stopImmediatePropagation__").unwrap();
    let value = v8::Boolean::new(scope, true);
    event_obj.set(scope, key.into(), value.into());

    // Also stop propagation
    let prop_key = v8::String::new(scope, "__stopPropagation__").unwrap();
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

    // Only allow preventDefault if cancelable
    let cancelable_key = v8::String::new(scope, "cancelable").unwrap();
    let is_cancelable = event_obj
        .get(scope, cancelable_key.into())
        .map(|v| v.is_true())
        .unwrap_or(false);

    if is_cancelable {
        let key = v8::String::new(scope, "__defaultPrevented__").unwrap();
        let value = v8::Boolean::new(scope, true);
        event_obj.set(scope, key.into(), value.into());
    }
}
