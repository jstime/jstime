// High-performance URL implementation matching Deno's architecture
// URLs stored in Rust, accessed via lazy getters

use ada_url::Url;
use std::cell::RefCell;
use std::collections::HashMap;

// Global URL storage - URLs stored by ID in Rust
thread_local! {
    static URL_STORAGE: RefCell<HashMap<u32, Url>> = RefCell::new(HashMap::new());
    static NEXT_URL_ID: RefCell<u32> = RefCell::new(1);
}

// Helper to allocate a new URL ID
fn allocate_url_id() -> u32 {
    NEXT_URL_ID.with(|id| {
        let mut current = id.borrow_mut();
        let new_id = *current;
        *current += 1;
        new_id
    })
}

// Helper to store URL and return ID
fn store_url(url: Url) -> u32 {
    let id = allocate_url_id();
    URL_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, url);
    });
    id
}

// Helper to get URL by ID
fn get_url(id: u32) -> Option<Url> {
    URL_STORAGE.with(|storage| {
        storage.borrow().get(&id).cloned()
    })
}

// Helper to update URL by ID
fn update_url(id: u32, url: Url) {
    URL_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, url);
    });
}

// Helper to convert Rust string to V8 string
fn to_v8_string<'s>(scope: &mut v8::PinScope<'s, '_>, s: &str) -> v8::Local<'s, v8::String> {
    v8::String::new(scope, s).unwrap()
}

// Helper to convert V8 value to Rust string
fn to_rust_string(scope: &mut v8::PinScope, val: v8::Local<v8::Value>) -> String {
    val.to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope)
}

// URL parse function - returns URL ID
fn url_parse(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() == 0 {
        rv.set(v8::null(scope).into());
        return;
    }

    let url_str = to_rust_string(scope, args.get(0));
    
    let base = if args.length() > 1 && !args.get(1).is_undefined() {
        Some(to_rust_string(scope, args.get(1)))
    } else {
        None
    };

    let parsed = if let Some(base_str) = base {
        Url::parse(&url_str, Some(&base_str))
    } else {
        Url::parse(&url_str, None)
    };

    match parsed {
        Ok(url) => {
            let url_id = store_url(url);
            let id_value = v8::Number::new(scope, url_id as f64);
            rv.set(id_value.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

// Get URL property by ID
fn url_get_property(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 2 {
        rv.set(v8::undefined(scope).into());
        return;
    }

    let id = args.get(0).number_value(scope).unwrap() as u32;
    let prop = to_rust_string(scope, args.get(1));

    if let Some(url) = get_url(id) {
        let value = match prop.as_str() {
            "href" => url.as_str(),
            "origin" => &url.origin(),
            "protocol" => &url.protocol(),
            "username" => &url.username(),
            "password" => &url.password(),
            "host" => &url.host(),
            "hostname" => &url.hostname(),
            "port" => &url.port(),
            "pathname" => &url.pathname(),
            "search" => &url.search(),
            "hash" => &url.hash(),
            _ => "",
        };
        let v8_str = to_v8_string(scope, value);
        rv.set(v8_str.into());
    } else {
        rv.set(v8::undefined(scope).into());
    }
}

// Set URL property by ID
fn url_set_property(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 3 {
        rv.set(v8::Boolean::new(scope, false).into());
        return;
    }

    let id = args.get(0).number_value(scope).unwrap() as u32;
    let prop = to_rust_string(scope, args.get(1));
    let value = to_rust_string(scope, args.get(2));

    if let Some(mut url) = get_url(id) {
        match prop.as_str() {
            "href" => {
                if let Ok(new_url) = Url::parse(&value, None) {
                    update_url(id, new_url);
                    rv.set(v8::Boolean::new(scope, true).into());
                    return;
                }
            }
            "protocol" => {
                url.set_protocol(&value);
                update_url(id, url);
                rv.set(v8::Boolean::new(scope, true).into());
                return;
            }
            "pathname" => {
                let _ = url.set_pathname(Some(&value));
                update_url(id, url);
                rv.set(v8::Boolean::new(scope, true).into());
                return;
            }
            "search" => {
                if value.is_empty() || value == "?" {
                    url.set_search(None);
                } else {
                    url.set_search(Some(&value));
                }
                update_url(id, url);
                rv.set(v8::Boolean::new(scope, true).into());
                return;
            }
            "hash" => {
                if value.is_empty() || value == "#" {
                    url.set_hash(None);
                } else {
                    url.set_hash(Some(&value));
                }
                update_url(id, url);
                rv.set(v8::Boolean::new(scope, true).into());
                return;
            }
            _ => {}
        }
    }
    
    rv.set(v8::Boolean::new(scope, false).into());
}

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_parse),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_property),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_property),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let key = to_v8_string(scope, "urlParse");
    let val = v8::Function::new(scope, url_parse).unwrap();
    bindings.set(scope, key.into(), val.into());

    let key = to_v8_string(scope, "urlGetProperty");
    let val = v8::Function::new(scope, url_get_property).unwrap();
    bindings.set(scope, key.into(), val.into());

    let key = to_v8_string(scope, "urlSetProperty");
    let val = v8::Function::new(scope, url_set_property).unwrap();
    bindings.set(scope, key.into(), val.into());
}
