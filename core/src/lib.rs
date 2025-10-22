mod builtins;
mod error;
mod event_loop;
mod isolate_state;
mod js_loading;
mod module;
mod script;

pub(crate) use isolate_state::IsolateState;

pub fn init(v8_flags: Option<Vec<String>>) {
    let mut flags = v8_flags.unwrap_or_default();

    // Add performance-oriented V8 flags if not already present
    // Only add flags that don't conflict with user-provided flags
    let perf_flags = [
        "--turbofan", // Enable TurboFan optimizing compiler (usually on by default)
        "--opt",      // Enable optimizations
    ];

    for flag in &perf_flags {
        if !flags
            .iter()
            .any(|f| f.starts_with(flag) || f.starts_with(&format!("--no-{}", &flag[2..])))
        {
            flags.push(flag.to_string());
        }
    }

    flags.push("jstime".to_owned());
    flags.rotate_right(1);

    v8::V8::set_flags_from_command_line(flags);

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}

/// Options for `JSTime::new`.
#[derive(Default)]
pub struct Options {
    // pub snapshot: Option<&'static [u8]>,
    // taking_snapshot: bool,
    pub process_argv: Vec<String>,
}

impl Options {
    pub fn new(_snapshot: Option<&'static [u8]>) -> Options {
        Options {
            // snapshot,
            // ..Options::default()
            process_argv: Vec::new(),
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
        let create_params = v8::Isolate::create_params()
            .external_references(builtins::get_external_references().into());
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

    fn create(options: Options, mut isolate: v8::OwnedIsolate) -> JSTime {
        // Set up import.meta callback before creating context
        isolate.set_host_initialize_import_meta_object_callback(
            module::host_initialize_import_meta_object_callback,
        );

        let global_context = {
            v8::scope!(let scope, &mut isolate);
            let context = v8::Context::new(scope, Default::default());
            let isolate_ref: &v8::Isolate = scope;
            v8::Global::new(isolate_ref, context)
        };

        isolate.set_slot(IsolateState::new(global_context, options.process_argv));

        // If snapshot data was provided, the builtins already exist within it.
        if true {
            let context = IsolateState::get(&mut isolate).borrow().context();
            v8::scope!(let scope, &mut isolate);
            let context_local = v8::Local::new(scope, context);
            let mut scope = v8::ContextScope::new(scope, context_local);
            builtins::Builtins::create(&mut scope);
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
        let result = {
            let context = IsolateState::get(self.isolate()).borrow().context();
            v8::scope!(let scope, self.isolate());
            let context_local = v8::Local::new(scope, context);
            let mut scope = v8::ContextScope::new(scope, context_local);
            let loader = module::Loader::new();

            let mut cwd = std::env::current_dir().unwrap();
            cwd.push("jstime");
            let cwd = cwd.into_os_string().into_string().unwrap();
            match loader.import(&mut scope, &cwd, filename) {
                Ok(_) => Ok(()),
                Err(exception) => {
                    // Format the exception value directly
                    let isolate: &v8::Isolate = &scope;
                    let exception_str = exception
                        .to_string(&scope)
                        .map(|s| s.to_rust_string_lossy(isolate))
                        .unwrap_or_else(|| "Unknown error".to_string());

                    // Try to get stack property for more details
                    if let Ok(exception_obj) = v8::Local::<v8::Object>::try_from(exception) {
                        let stack_key = v8::String::new(&scope, "stack").unwrap();
                        if let Some(stack_val) = exception_obj.get(&scope, stack_key.into())
                            && let Some(stack_str) = stack_val.to_string(&scope)
                        {
                            let stack = stack_str.to_rust_string_lossy(isolate);
                            if !stack.is_empty() && stack != exception_str {
                                return Err(stack);
                            }
                        }
                    }

                    // Remove "Error: " prefix if present (V8 adds this when creating Error objects)
                    let exception_str = if let Some(stripped) = exception_str.strip_prefix("Error: ")
                    {
                        stripped.to_string()
                    } else {
                        exception_str
                    };

                    Err(exception_str)
                }
            }
        };

        // Run the event loop to process any pending timers
        self.run_event_loop();

        result
    }

    /// Run a script and get a string representation of the result.
    /// This version runs the event loop after execution, which is suitable for file execution.
    /// For REPL usage, use `run_script_no_event_loop` instead.
    pub fn run_script(&mut self, source: &str, filename: &str) -> Result<String, String> {
        let result = self.run_script_no_event_loop(source, filename);

        // Run the event loop to process any pending timers
        self.run_event_loop();

        result
    }

    /// Run a script and get a string representation of the result without running the event loop.
    /// This is suitable for REPL usage where the event loop should not block between commands.
    pub fn run_script_no_event_loop(
        &mut self,
        source: &str,
        filename: &str,
    ) -> Result<String, String> {
        let context = IsolateState::get(self.isolate()).borrow().context();
        v8::scope!(let scope, self.isolate());
        let context_local = v8::Local::new(scope, context);
        let mut scope = v8::ContextScope::new(scope, context_local);
        match script::run(&mut scope, source, filename) {
            Ok(v) => {
                let isolate: &v8::Isolate = &scope;
                Ok(v.to_string(&scope).unwrap().to_rust_string_lossy(isolate))
            }
            Err(e) => Err(e),
        }
    }

    /// Tick the event loop to execute ready timers without blocking.
    /// This is suitable for REPL usage to allow timers to execute in the background.
    pub fn tick_event_loop(&mut self) {
        let context = IsolateState::get(self.isolate()).borrow().context();
        v8::scope!(let scope, self.isolate());
        let context_local = v8::Local::new(scope, context);
        let mut scope = v8::ContextScope::new(scope, context_local);
        let event_loop = event_loop::get_event_loop(&mut scope);
        event_loop.borrow_mut().tick(&mut scope);
    }

    /// Run the event loop until all pending operations are complete
    fn run_event_loop(&mut self) {
        let context = IsolateState::get(self.isolate()).borrow().context();
        v8::scope!(let scope, self.isolate());
        let context_local = v8::Local::new(scope, context);
        let mut scope = v8::ContextScope::new(scope, context_local);
        let event_loop = event_loop::get_event_loop(&mut scope);
        event_loop.borrow_mut().run(&mut scope);
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
