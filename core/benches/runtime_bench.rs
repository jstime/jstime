use criterion::{Criterion, black_box, criterion_group, criterion_main};
use jstime_core::{self as jstime, Options};

fn setup() -> jstime::JSTime {
    jstime::init(None);
    let options = Options::default();
    jstime::JSTime::new(options)
}

fn bench_startup(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup");

    // Initialize V8 once before benchmarking
    jstime::init(None);

    // Benchmark creating a new JSTime instance (with snapshot)
    group.bench_function("new_instance_with_snapshot", |b| {
        b.iter(|| {
            let options = Options::default();
            black_box(jstime::JSTime::new(options))
        })
    });

    // Benchmark creating an instance without snapshot (using None)
    group.bench_function("new_instance_without_snapshot", |b| {
        b.iter(|| {
            let options = Options::new(None);
            black_box(jstime::JSTime::new(options))
        })
    });

    // Benchmark creating an instance and running a simple script
    group.bench_function("new_instance_and_hello_world", |b| {
        b.iter(|| {
            let options = Options::default();
            let mut js = jstime::JSTime::new(options);
            black_box(js.run_script("'hello world'", "bench.js"))
        })
    });

    // Benchmark cold start with builtin usage
    group.bench_function("new_instance_with_builtins", |b| {
        b.iter(|| {
            let options = Options::default();
            let mut js = jstime::JSTime::new(options);
            black_box(js.run_script(
                "console.log('test'); performance.now(); new URL('https://example.com')",
                "bench.js",
            ))
        })
    });

    group.finish();
}

fn bench_script_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("script_execution");

    // Simple arithmetic
    group.bench_function("simple_arithmetic", |b| {
        b.iter_batched(
            setup,
            |mut js| js.run_script(black_box("1 + 1"), "bench.js"),
            criterion::BatchSize::SmallInput,
        )
    });

    // String operations
    group.bench_function("string_concat", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("let s = ''; for(let i = 0; i < 100; i++) s += 'x';"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // Array operations
    group.bench_function("array_operations", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("Array.from({length: 1000}, (_, i) => i).reduce((a, b) => a + b, 0)"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // Object creation
    group.bench_function("object_creation", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("const objs = []; for(let i = 0; i < 100; i++) objs.push({x: i, y: i * 2});"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // Function calls
    group.bench_function("function_calls", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("function add(a, b) { return a + b; } let sum = 0; for(let i = 0; i < 1000; i++) sum = add(sum, i);"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_console_api(c: &mut Criterion) {
    let mut group = c.benchmark_group("console_api");

    group.bench_function("console_log", |b| {
        b.iter_batched(
            setup,
            |mut js| js.run_script(black_box("console.log('hello', 'world')"), "bench.js"),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("console_multiple_calls", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("for(let i = 0; i < 10; i++) console.log('iteration', i);"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_json_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_operations");

    group.bench_function("json_stringify", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("const obj = {a: 1, b: 'test', c: [1, 2, 3], d: {nested: true}}; JSON.stringify(obj);"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("json_parse", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box(
                        r#"JSON.parse('{"a":1,"b":"test","c":[1,2,3],"d":{"nested":true}}')"#,
                    ),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("json_round_trip", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("const obj = {a: 1, b: 'test'}; JSON.parse(JSON.stringify(obj));"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_performance_api(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_api");

    group.bench_function("performance_now", |b| {
        b.iter_batched(
            setup,
            |mut js| js.run_script(black_box("performance.now()"), "bench.js"),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("performance_now_loop", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("for(let i = 0; i < 100; i++) performance.now();"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_base64_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("base64_operations");

    group.bench_function("btoa", |b| {
        b.iter_batched(
            setup,
            |mut js| js.run_script(black_box("btoa('Hello, World!')"), "bench.js"),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("atob", |b| {
        b.iter_batched(
            setup,
            |mut js| js.run_script(black_box("atob('SGVsbG8sIFdvcmxkIQ==')"), "bench.js"),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("base64_round_trip", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("atob(btoa('Test data for encoding'))"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_url_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("url_operations");

    group.bench_function("url_parse", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("new URL('https://example.com/path?query=value')"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("url_searchparams", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box(
                        "const params = new URLSearchParams('a=1&b=2&c=3'); params.get('b');",
                    ),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_crypto_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("crypto_operations");

    group.bench_function("random_uuid", |b| {
        b.iter_batched(
            setup,
            |mut js| js.run_script(black_box("crypto.randomUUID()"), "bench.js"),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("random_values", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("crypto.getRandomValues(new Uint8Array(32))"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_event_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_operations");

    group.bench_function("event_create", |b| {
        b.iter_batched(
            setup,
            |mut js| js.run_script(black_box("new Event('test')"), "bench.js"),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("event_dispatch", |b| {
        b.iter_batched(
            setup,
            |mut js| {
                js.run_script(
                    black_box("const target = new EventTarget(); target.addEventListener('test', () => {}); target.dispatchEvent(new Event('test'));"),
                    "bench.js",
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_startup,
    bench_script_execution,
    bench_console_api,
    bench_json_operations,
    bench_performance_api,
    bench_base64_operations,
    bench_url_operations,
    bench_crypto_operations,
    bench_event_operations
);
criterion_main!(benches);
