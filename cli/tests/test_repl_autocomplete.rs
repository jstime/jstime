use rustc_hash::FxHashSet;
use std::sync::Once;
use std::sync::{Arc, RwLock};

#[allow(dead_code)]
#[path = "../repl_autocomplete.rs"]
mod repl_autocomplete;

static INIT: Once = Once::new();

fn test_runtime() -> jstime_core::JSTime {
    INIT.call_once(|| {
        jstime_core::init(None);
    });

    let options = jstime_core::Options::new(Some(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/snapshot_data.blob"
    ))));
    jstime_core::JSTime::new(options)
}

#[test]
fn extract_repl_binding_names_tracks_top_level_declarations() {
    let bindings = repl_autocomplete::extract_repl_binding_names(
        "const myObj = { foo: 1 }; let myValue = 1; class MyClass {}; async function myFn() {}",
    );

    assert_eq!(bindings, vec!["MyClass", "myFn", "myObj", "myValue"]);
}

#[test]
fn extract_repl_binding_names_ignores_nested_declarations() {
    let bindings = repl_autocomplete::extract_repl_binding_names(
        "for (let i = 0; i < 1; i++) { const inner = i; } if (true) { let other = 1; }",
    );

    assert!(bindings.is_empty());
}

#[test]
fn refresh_cache_includes_repl_lexical_bindings() {
    let mut runtime = test_runtime();
    runtime
        .run_script_no_event_loop(
            "const myObj = { foo: 1, bar: 2 }; let myValue = 1; class MyClass {}",
            "test",
        )
        .unwrap();

    let cache = Arc::new(RwLock::new(repl_autocomplete::CompletionCache::new()));
    let repl_bindings = repl_autocomplete::extract_repl_binding_names(
        "const myObj = { foo: 1, bar: 2 }; let myValue = 1; class MyClass {}",
    )
    .into_iter()
    .collect::<FxHashSet<_>>();

    repl_autocomplete::refresh_cache(&mut runtime, &cache, &repl_bindings);

    let cache_guard = cache.read().unwrap();
    assert!(cache_guard.globals.contains(&"myObj".to_string()));
    assert!(cache_guard.globals.contains(&"myValue".to_string()));
    assert!(cache_guard.globals.contains(&"MyClass".to_string()));

    let props = cache_guard.properties.get("myObj").unwrap();
    assert!(props.contains(&"foo".to_string()));
    assert!(props.contains(&"bar".to_string()));
}

#[test]
fn extract_repl_binding_names_supports_dollar_identifiers() {
    let bindings = repl_autocomplete::extract_repl_binding_names(
        "const $myObj = { value: 1 }; let $value = 1;",
    );

    assert_eq!(bindings, vec!["$myObj", "$value"]);
}
