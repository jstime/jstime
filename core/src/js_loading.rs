use rusty_v8 as v8;

// Common code used by both `module.rs` and `script.rs`

pub(crate) fn create_script_origin<'s>(
    scope: &mut v8::HandleScope<'s, ()>,
    filepath: v8::Local<'s, v8::String>,
    is_module: bool,
) -> v8::ScriptOrigin<'s> {
    let resource_name = filepath.into();
    let resource_line_offset = 0;
    let resource_column_offset = 0;
    let resource_is_shared_cross_origin = false;
    let script_id = 0;
    let source_map_url = v8::String::new(scope, "soure_map_url").unwrap().into();
    let resource_is_opaque = true;
    let is_wasm = false;

    v8::ScriptOrigin::new(
        scope,
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
