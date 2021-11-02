use crate::js_loading;

pub(crate) fn run<'s>(
    scope: &mut v8::HandleScope<'s>,
    js: &str,
    filepath: &str,
) -> Result<v8::Local<'s, v8::Value>, v8::Local<'s, v8::Value>> {
    let scope = &mut v8::TryCatch::new(scope);

    let filepath = v8::String::new(scope, filepath).unwrap();
    let origin = js_loading::create_script_origin(scope, filepath, false);

    let code = v8::String::new(scope, js).unwrap();

    v8::Script::compile(scope, code, Some(&origin))
        .and_then(|script| script.run(scope))
        .map_or_else(|| Err(scope.stack_trace().unwrap()), Ok)
}
