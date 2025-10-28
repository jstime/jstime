use crate::js_loading;

pub(crate) fn run<'s>(
    scope: &mut v8::PinScope<'s, '_>,
    js: &str,
    filepath: &str,
) -> Result<v8::Local<'s, v8::Value>, String> {
    v8::tc_scope!(let tc, scope);

    let filepath = v8::String::new(tc, filepath).unwrap();
    let origin = js_loading::create_script_origin(tc, filepath, false);

    let code = v8::String::new(tc, js).unwrap();

    let script = v8::Script::compile(tc, code, Some(&origin));
    let result = script.and_then(|script| script.run(tc));

    match result {
        Some(value) => Ok(value),
        None => {
            if tc.has_caught() {
                // Format the error with detailed information
                Err(crate::error::format_exception(tc))
            } else {
                panic!("Script execution failed without exception")
            }
        }
    }
}
