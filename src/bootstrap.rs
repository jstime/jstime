use rusty_v8 as v8;

pub(crate) fn init() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}

pub fn set_flags(args: Vec<String>) -> Vec<String> {
    v8::V8::set_flags_from_command_line(args)
}

pub(crate) fn set_globals(scope: &mut v8::HandleScope) {
    crate::builtins::Builtins::create(scope);
}
