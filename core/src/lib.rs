#[macro_use]
extern crate lazy_static;

mod builtins;
mod isolate_state;
mod js_loading;
mod module;
mod script;

pub(crate) use isolate_state::IsolateState;

pub fn init(v8_flags: Option<Vec<String>>) {
    if let Some(mut v8_flags) = v8_flags {
        v8_flags.push("jstime".to_owned());
        v8_flags.rotate_right(1);

        v8::V8::set_flags_from_command_line(v8_flags);
    }

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}

/// Options for `JSTime::new`.
#[derive(Default)]
pub struct Options {
    // pub snapshot: Option<&'static [u8]>,
    // taking_snapshot: bool,
}

impl Options {
    pub fn new(_snapshot: Option<&'static [u8]>) -> Options {
        Options {
            // snapshot,
            ..Options::default()
        }
    }
}

/// JSTime Instance.
#[allow(clippy::all)]
pub struct JSTime {
    isolate: Option<v8::OwnedIsolate>,
    // taking_snapshot: bool,
}

impl JSTime {
    /// Create a new JSTime instance from `options`.
    pub fn new(options: Options) -> JSTime {
        let create_params =
            v8::Isolate::create_params().external_references(&**builtins::EXTERNAL_REFERENCES);
        // if let Some(snapshot) = options.snapshot {
            // create_params = create_params.snapshot_blob(snapshot);
        // }
        let isolate = v8::Isolate::new(create_params);
        JSTime::create(options, isolate)
    }

    // pub fn create_snapshot(mut options: Options) -> Vec<u8> {
    //     assert!(
    //         options.snapshot.is_none(),
    //         "Cannot pass snapshot data while creating snapshot"
    //     );
    //     options.taking_snapshot = true;

    //     let mut s = v8::SnapshotCreator::new(Some(&builtins::EXTERNAL_REFERENCES));

    //     {
    //         let mut jstime = JSTime::create(options, unsafe { s.get_owned_isolate() });
    //         {
    //             let context = IsolateState::get(jstime.isolate()).borrow().context();
    //             let scope = &mut v8::HandleScope::new(jstime.isolate());
    //             let context = v8::Local::new(scope, context);
    //             s.set_default_context(context);
    //         }
    //         // Context needs to be dropped before create_blob
    //         IsolateState::get(jstime.isolate())
    //             .borrow_mut()
    //             .drop_context();
    //     }

    //     match s.create_blob(v8::FunctionCodeHandling::Keep) {
    //         Some(data) => data.to_owned(),
    //         None => {
    //             // dropping SnapshotCreator will panic if it failed, and
    //             // we're going to panic here anyway, so just forget it.
    //             std::mem::forget(s);
    //             panic!("Unable to create snapshot");
    //         }
    //     }
    // }

    fn create(_options: Options, mut isolate: v8::OwnedIsolate) -> JSTime {
        let global_context = {
            let scope = &mut v8::HandleScope::new(&mut isolate);
            let context = v8::Context::new(scope);
            v8::Global::new(scope, context)
        };

        isolate.set_slot(IsolateState::new(global_context));

        // If snapshot data was provided, the builtins already exist within it.
        if true {
            let context = IsolateState::get(&mut isolate).borrow().context();
            let scope = &mut v8::HandleScope::with_context(&mut isolate, context);
            builtins::Builtins::create(scope);
        }

        JSTime {
            isolate: Some(isolate),
            // taking_snapshot: options.taking_snapshot,
        }
    }

    fn isolate(&mut self) -> &mut v8::Isolate {
        match self.isolate.as_mut() {
            Some(i) => i,
            None => unsafe {
                std::hint::unreachable_unchecked();
            },
        }
    }

    /// Import a module by filename.
    pub fn import(&mut self, filename: &str) -> Result<(), String> {
        let context = IsolateState::get(self.isolate()).borrow().context();
        let scope = &mut v8::HandleScope::with_context(self.isolate(), context);
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
        let context = IsolateState::get(self.isolate()).borrow().context();
        let scope = &mut v8::HandleScope::with_context(self.isolate(), context);
        match script::run(scope, source, filename) {
            Ok(v) => Ok(v.to_string(scope).unwrap().to_rust_string_lossy(scope)),
            Err(e) => Err(e.to_string(scope).unwrap().to_rust_string_lossy(scope)),
        }
    }
}

impl Drop for JSTime {
    fn drop(&mut self) {
        // if self.taking_snapshot {
            // The isolate is not actually owned by JSTime if we're
            // snapshotting, it's owned by the SnapshotCreator.
            // std::mem::forget(self.isolate.take().unwrap())
        // }
    }
}
