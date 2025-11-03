mod builtins;
mod error;
mod event_loop;
mod isolate_state;
mod js_loading;
mod module;
mod pool;
mod script;
mod sourcemap;

pub(crate) use isolate_state::IsolateState;

pub fn init(v8_flags: Option<Vec<String>>) {
    // Initialize ICU data before V8 initialization
    // This is required for locale-specific operations like toLocaleString()
    static ICU_INIT: std::sync::Once = std::sync::Once::new();
    ICU_INIT.call_once(|| {
        let icu_data =
            align_data::include_aligned!(align_data::Align16, "../third_party/icu/icudtl.dat");
        // Ignore errors - ICU data initialization is best-effort
        let _ = v8::icu::set_common_data_74(icu_data);
        // Set default locale to en_US
        v8::icu::set_default_locale("en_US");
    });

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

    static V8_INIT: std::sync::Once = std::sync::Once::new();
    V8_INIT.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}

/// Options for `JSTime::new`.
#[derive(Default)]
pub struct Options {
    pub snapshot: Option<&'static [u8]>,
    taking_snapshot: bool,
    pub process_argv: Vec<String>,
    /// Number of warmup iterations to run before actual execution.
    /// This allows V8's TurboFan JIT compiler to optimize the code.
    /// Default is 0 (no warmup).
    pub warmup_iterations: usize,
}

impl Options {
    pub fn new(snapshot: Option<&'static [u8]>) -> Options {
        Options {
            snapshot,
            taking_snapshot: false,
            process_argv: Vec::new(),
            warmup_iterations: 0,
        }
    }

    pub fn with_process_argv(mut self, argv: Vec<String>) -> Self {
        self.process_argv = argv;
        self
    }

    pub fn with_warmup(mut self, iterations: usize) -> Self {
        self.warmup_iterations = iterations;
        self
    }
}

/// JSTime Instance.
#[allow(clippy::all)]
pub struct JSTime {
    isolate: Option<v8::OwnedIsolate>,
    taking_snapshot: bool,
    warmup_iterations: usize,
}

impl JSTime {
    /// Create a new JSTime instance from `options`.
    pub fn new(options: Options) -> JSTime {
        let mut create_params = v8::Isolate::create_params()
            .external_references(builtins::get_external_references().into_vec().into())
            .heap_limits(0, 1024 * 1024 * 1024); // 1GB max heap size
        if let Some(snapshot) = options.snapshot {
            create_params = create_params.snapshot_blob(snapshot.into());
        }
        let isolate = v8::Isolate::new(create_params);
        JSTime::create(options, isolate)
    }

    pub fn create_snapshot(mut options: Options) -> Vec<u8> {
        assert!(
            options.snapshot.is_none(),
            "Cannot pass snapshot data while creating snapshot"
        );
        options.taking_snapshot = true;

        let external_refs = builtins::get_external_references();
        let external_refs_cow: std::borrow::Cow<'static, [v8::ExternalReference]> =
            std::borrow::Cow::Owned(external_refs.into_vec());
        let mut isolate = v8::Isolate::snapshot_creator(Some(external_refs_cow), None);

        // Set up import.meta callback before creating context
        isolate.set_host_initialize_import_meta_object_callback(
            module::host_initialize_import_meta_object_callback,
        );

        // Set up dynamic import callback
        isolate.set_host_import_module_dynamically_callback(
            module::host_import_module_dynamically_callback,
        );

        let global_context = {
            v8::scope!(let scope, &mut isolate);
            let context = v8::Context::new(scope, Default::default());
            let isolate_ref: &v8::Isolate = scope;
            v8::Global::new(isolate_ref, context)
        };

        isolate.set_slot(IsolateState::new(global_context, options.process_argv));

        // Create builtins in the snapshot context and set default context
        {
            let context = IsolateState::get(&mut isolate).borrow().context();
            v8::scope!(let scope, &mut isolate);
            let context_local = v8::Local::new(scope, context);
            let scope = &mut v8::ContextScope::new(scope, context_local);

            // Create builtins
            builtins::Builtins::create(scope);

            // Set the default context for the snapshot
            scope.set_default_context(context_local);
        }

        // Drop the context before creating the blob
        IsolateState::get(&mut isolate).borrow_mut().drop_context();

        match isolate.create_blob(v8::FunctionCodeHandling::Keep) {
            Some(data) => data.to_vec(),
            None => {
                panic!("Unable to create snapshot");
            }
        }
    }

    fn create(options: Options, mut isolate: v8::OwnedIsolate) -> JSTime {
        // Set up import.meta callback before creating context
        isolate.set_host_initialize_import_meta_object_callback(
            module::host_initialize_import_meta_object_callback,
        );

        // Set up dynamic import callback
        isolate.set_host_import_module_dynamically_callback(
            module::host_import_module_dynamically_callback,
        );

        let global_context = {
            v8::scope!(let scope, &mut isolate);
            let context = v8::Context::new(scope, Default::default());
            let isolate_ref: &v8::Isolate = scope;
            v8::Global::new(isolate_ref, context)
        };

        isolate.set_slot(IsolateState::new(global_context, options.process_argv));

        // If snapshot data was provided, the builtins already exist within it.
        if options.snapshot.is_none() {
            let context = IsolateState::get(&mut isolate).borrow().context();
            v8::scope!(let scope, &mut isolate);
            let context_local = v8::Local::new(scope, context);
            let mut scope = v8::ContextScope::new(scope, context_local);
            builtins::Builtins::create(&mut scope);
        }

        JSTime {
            isolate: Some(isolate),
            taking_snapshot: options.taking_snapshot,
            warmup_iterations: options.warmup_iterations,
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
        // Perform JIT warmup if configured
        if self.warmup_iterations > 0 {
            self.warmup_import(filename)?;
        }

        let result = {
            let context = IsolateState::get(self.isolate()).borrow().context();
            v8::scope!(let scope, self.isolate());
            let context_local = v8::Local::new(scope, context);
            let mut scope = v8::ContextScope::new(scope, context_local);

            // Use a TryCatch scope to properly capture error details
            v8::tc_scope!(let tc, &mut scope);
            let loader = module::Loader::new();

            let mut cwd = std::env::current_dir().unwrap();
            cwd.push("jstime");
            let cwd = cwd.into_os_string().into_string().unwrap();
            match loader.import(tc, &cwd, filename) {
                Ok(_) => Ok(()),
                Err(exception) => {
                    // If we have caught exception details, format them properly
                    if tc.has_caught() {
                        Err(crate::error::format_exception(tc))
                    } else {
                        // Fallback: Format the exception value directly with enhanced formatting
                        Err(crate::error::format_exception_value(tc, exception))
                    }
                }
            }
        };

        // Run the event loop to process any pending timers
        self.run_event_loop();

        result
    }

    /// Warm up the JIT compiler by importing the module multiple times.
    /// This allows V8's TurboFan compiler to optimize the module code.
    fn warmup_import(&mut self, filename: &str) -> Result<(), String> {
        for _ in 0..self.warmup_iterations {
            let context = IsolateState::get(self.isolate()).borrow().context();
            v8::scope!(let scope, self.isolate());
            let context_local = v8::Local::new(scope, context);
            let mut scope = v8::ContextScope::new(scope, context_local);
            v8::tc_scope!(let tc, &mut scope);
            let loader = module::Loader::new();
            let mut cwd = std::env::current_dir().unwrap();
            cwd.push("jstime");
            let cwd = cwd.into_os_string().into_string().unwrap();
            // Import but ignore result during warmup
            match loader.import(tc, &cwd, filename) {
                Ok(_) => {}
                Err(exception) => {
                    if tc.has_caught() {
                        return Err(crate::error::format_exception(tc));
                    } else {
                        return Err(crate::error::format_exception_value(tc, exception));
                    }
                }
            }
        }
        Ok(())
    }

    /// Run a script and get a string representation of the result.
    /// This version runs the event loop after execution, which is suitable for file execution.
    /// For REPL usage, use `run_script_no_event_loop` instead.
    pub fn run_script(&mut self, source: &str, filename: &str) -> Result<String, String> {
        // Perform JIT warmup if configured
        if self.warmup_iterations > 0 {
            self.warmup_script(source, filename)?;
        }

        let result = self.run_script_no_event_loop(source, filename);

        // Run the event loop to process any pending timers
        self.run_event_loop();

        result
    }

    /// Warm up the JIT compiler by running the script multiple times.
    /// This allows V8's TurboFan compiler to optimize the code before the actual execution.
    fn warmup_script(&mut self, source: &str, filename: &str) -> Result<(), String> {
        for _ in 0..self.warmup_iterations {
            let context = IsolateState::get(self.isolate()).borrow().context();
            v8::scope!(let scope, self.isolate());
            let context_local = v8::Local::new(scope, context);
            let mut scope = v8::ContextScope::new(scope, context_local);
            // Run script but ignore the result during warmup
            script::run(&mut scope, source, filename)?;
        }
        Ok(())
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
        if self.taking_snapshot {
            // The isolate is not actually owned by JSTime if we're
            // snapshotting, it's owned by the SnapshotCreator.
            std::mem::forget(self.isolate.take().unwrap())
        }
    }
}
