use rusty_v8 as v8;

// Common code used by both `module.rs` and `script.rs`

pub(crate) fn create_script_origin<'s>(
    scope: &mut v8::HandleScope<'s, ()>,
    filepath: v8::Local<'s, v8::String>,
    is_module: bool,
) -> v8::ScriptOrigin<'s> {
    let resource_name = filepath.into();
    let resource_line_offset = v8::Integer::new(scope, 0);
    let resource_column_offset = v8::Integer::new(scope, 0);
    let resource_is_shared_cross_origin = v8::Boolean::new(scope, false);
    let script_id = v8::Integer::new(scope, 0);
    let source_map_url = v8::String::new(scope, "").unwrap().into();
    let resource_is_opaque = v8::Boolean::new(scope, true);
    let is_wasm = v8::Boolean::new(scope, false);
    let is_module = v8::Boolean::new(scope, is_module);

    v8::ScriptOrigin::new(
        resource_name,
        resource_line_offset,
        resource_column_offset,
        resource_is_shared_cross_origin,
        script_id,
        source_map_url,
        resource_is_opaque,
        is_wasm,
        is_module,
    )
}
