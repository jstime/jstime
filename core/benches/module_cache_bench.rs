use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jstime_core as jstime;
use std::fs;
use std::path::PathBuf;

fn setup_test_modules() -> (PathBuf, PathBuf, PathBuf) {
    let temp_dir = std::env::temp_dir().join("jstime_bench");
    fs::create_dir_all(&temp_dir).ok();

    let shared = temp_dir.join("shared_bench.js");
    let module_a = temp_dir.join("module_a_bench.js");
    let module_b = temp_dir.join("module_b_bench.js");

    fs::write(
        &shared,
        "export const value = 'shared data'; export function compute() { return 42; }",
    )
    .unwrap();
    fs::write(
        &module_a,
        "import { value } from './shared_bench.js'; export const a = value;",
    )
    .unwrap();
    fs::write(
        &module_b,
        "import { compute } from './shared_bench.js'; export const b = compute();",
    )
    .unwrap();

    (shared, module_a, module_b)
}

fn cleanup_test_modules(shared: &PathBuf, module_a: &PathBuf, module_b: &PathBuf) {
    fs::remove_file(shared).ok();
    fs::remove_file(module_a).ok();
    fs::remove_file(module_b).ok();
}

fn bench_module_loading_with_runtime_creation(c: &mut Criterion) {
    // Initialize V8 once at the start
    jstime::init(None);
    let (shared, module_a, module_b) = setup_test_modules();

    // Clear any existing cache to get baseline
    jstime::module::clear_source_cache();

    c.bench_function("module_loading_with_runtime_creation", |b| {
        b.iter(|| {
            let options = jstime::Options::default();
            let mut runtime = jstime::JSTime::new(options);
            runtime
                .import(black_box(module_a.to_str().unwrap()))
                .unwrap();
            runtime
                .import(black_box(module_b.to_str().unwrap()))
                .unwrap();
        });
    });

    cleanup_test_modules(&shared, &module_a, &module_b);
}

criterion_group!(benches, bench_module_loading_with_runtime_creation);
criterion_main!(benches);
