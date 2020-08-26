mod builtins;
mod isolate_state;
mod js_loading;
mod module;
mod script;

pub(crate) use isolate_state::IsolateState;
use rusty_v8 as v8;

pub fn init(v8_flags: Option<Vec<String>>) {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    if let Some(v8_flags) = v8_flags {
        v8::V8::set_flags_from_command_line(v8_flags);
    }
}

/// Options for `JSTime::new`.
#[derive(Default)]
pub struct Options {}

impl Options {
    pub fn new() -> Options {
        Options {}
    }
}

/// JSTime Instance.
pub struct JSTime {
    isolate: v8::OwnedIsolate,
}

impl JSTime {
    /// Create a new JSTime instance from `options`.
    pub fn new(_options: Options) -> JSTime {
        let mut isolate = v8::Isolate::new(Default::default());

        let global_context = {
            let scope = &mut v8::HandleScope::new(&mut isolate);
            let context = v8::Context::new(scope);
            v8::Global::new(scope, context)
        };

        isolate.set_slot(IsolateState::new(global_context));

        {
            let context = IsolateState::get(&mut isolate).borrow().context.clone();
            let scope = &mut v8::HandleScope::with_context(&mut isolate, context);
            builtins::Builtins::create(scope);
        }

        JSTime { isolate }
    }

    /// Import a module by filename.
    pub fn import(&mut self, filename: &str) -> Result<(), String> {
        let context = IsolateState::get(&mut self.isolate)
            .borrow()
            .context
            .clone();
        let scope = &mut v8::HandleScope::with_context(&mut self.isolate, context);
        let loader = module::Loader::new();

        let mut cwd = std::env::current_dir().unwrap();
        cwd.push("jstime");
        let cwd = cwd.into_os_string().into_string().unwrap();
        match loader.import(scope, &cwd, filename) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string(scope).unwrap().to_rust_string_lossy(scope)),
        }
    }

    /// Run a script and get a string representation of the result.
    pub fn run_script(&mut self, source: &str, filename: &str) -> Result<String, String> {
        let context = IsolateState::get(&mut self.isolate)
            .borrow()
            .context
            .clone();
        let scope = &mut v8::HandleScope::with_context(&mut self.isolate, context);
        match script::run(scope, source, filename) {
            Ok(v) => Ok(v.to_string(scope).unwrap().to_rust_string_lossy(scope)),
            Err(e) => Err(e.to_string(scope).unwrap().to_rust_string_lossy(scope)),
        }
    }
}
