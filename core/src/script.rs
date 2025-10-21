use crate::js_loading;

pub(crate) fn run<'s>(
    scope: &mut v8::PinScope<'s, '_>,
    js: &str,
    filepath: &str,
) -> Result<v8::Local<'s, v8::Value>, v8::Local<'s, v8::Value>> {
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
                // Try to get the stack trace first, fall back to exception
                if let Some(stack_trace) = tc.stack_trace() {
                    Err(stack_trace)
                } else {
                    Err(tc.exception().unwrap())
                }
            } else {
                panic!("Script execution failed without exception")
            }
        }
    }
}
