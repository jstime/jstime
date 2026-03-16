# AGENTS.md

This file is the canonical repository-wide guidance for coding agents working in `jstime`.
Use it as the default source of truth for project context, architecture, and validation expectations.

## Project overview

- `jstime` is a minimal, performant JavaScript runtime built on top of V8 and written in Rust.
- The workspace contains two main crates:
  - `jstime_core` (`core/`): the embeddable runtime library.
  - `jstime` (`cli/`): the CLI and REPL.
- The runtime focuses on a small, standards-oriented API surface with modern JavaScript features such as ES modules, top-level `await`, JSON modules, Web APIs, and WebAssembly.

## Important repository areas

- `core/src/builtins/`: built-in JavaScript APIs grouped by standards body:
  - `whatwg/`
  - `w3c/`
  - `node/`
  - `polyfills/`
- `core/src/js_loading.rs` and `core/src/module.rs`: module loading and resolution.
- `core/src/event_loop.rs`: async task and event loop implementation.
- `core/src/isolate_state.rs`: per-isolate runtime state.
- `cli/main.rs`: CLI entrypoint and REPL wiring.
- `cli/repl_autocomplete.rs`: REPL completion logic.
- `cli/build.rs`: build-time snapshot generation.
- `core/tests/`: integration, feature, and conformance tests.
- `docs/`: user-facing documentation.

## Rust and code style expectations

- Rust edition: `2024`.
- Keep changes minimal, focused, and consistent with the existing style.
- Prefer explicit error handling over panics in library code.
- Use descriptive error messages.
- Follow standard Rust naming conventions:
  - `snake_case` for functions, modules, and variables
  - `PascalCase` for types and traits
  - `SCREAMING_SNAKE_CASE` for constants
- Prefer `pub(crate)` for internal APIs that do not need wider visibility.
- Add `///` doc comments for public APIs when introducing or substantially changing them.
- Minimize `unsafe`; use it only where required for V8/FFI integration.

## Validation and CI expectations

Before considering work complete, use the smallest relevant checks first and broaden only as needed.

- Format: `cargo fmt --all -- --check`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Tests: `cargo test --locked`

CI currently runs formatting, clippy, and tests across `stable`, `beta`, and `nightly` toolchains.

## Working with V8

- Be careful with `HandleScope` and `ContextScope` lifetimes.
- Use `v8::Global` for JavaScript values that must outlive a local scope.
- Access per-isolate state through `IsolateState::get(...)`.
- Prefer existing V8 integration patterns already used in `core/src/` over inventing new ones.
- Be especially careful with ownership, raw pointers, and resource cleanup around FFI boundaries.

## Built-in APIs

Built-ins live in `core/src/builtins/` and are typically split into:

- a Rust implementation file such as `*_impl.rs`
- a JavaScript companion file such as `*.js`

When adding or extending a built-in API:

1. Put it in the appropriate standards bucket (`whatwg`, `w3c`, `node`, or `polyfills`).
2. Register it in `core/src/builtins/mod.rs` and any required initialization/snapshot plumbing.
3. Add or update tests in `core/tests/`.
4. Update the relevant docs in `docs/`.
5. Keep the implementation lightweight and aligned with the project's minimal-runtime goals.

### REPL autocomplete note

REPL autocomplete is now largely dynamic via `cli/repl_autocomplete.rs`.
Do not assume new built-ins require manual updates to `cli/main.rs`; verify whether the change actually needs explicit completion handling.

## Testing guidance

- Prefer targeted test runs before broader suites.
- Core test helpers live in `core/tests/common/mod.rs`.
- Conformance coverage is documented in `core/tests/CONFORMANCE_TESTS.md`.
- Use fixtures under `core/tests/fixtures/` when extending behavior that depends on modules, files, JSON, WASM, or other external assets.
- If you change CLI behavior, check `cli/tests/` in addition to core tests.

## Documentation expectations

Keep documentation aligned with behavior changes.

Common files to update include:

- `README.md`
- `docs/README.md`
- `docs/runtime.md`
- `docs/apis/web-apis.md`
- `docs/apis/encoding-crypto.md`
- `docs/apis/system.md`
- `docs/apis/modules.md`
- `docs/apis/error-handling.md`
- `ARCHITECTURE.md`
- `PERFORMANCE.md`

Update only the documentation relevant to the change; do not create speculative churn.

## Useful dependencies and patterns

- `v8`: JavaScript engine bindings
- `ada-url`: URL parsing
- `ureq`: HTTP client backing fetch functionality
- `rustc-hash`: fast hash map/set implementations used throughout the project
- `ring`: cryptographic operations
- `rustyline`: REPL line editing in the CLI
- `structopt`: CLI argument parsing

Prefer existing crate choices and established local patterns unless there is a strong reason to change them.

## Project principles

- Keep the runtime small and focused.
- Favor standards compliance where the project already implements a standard surface.
- Protect performance-sensitive paths.
- Maintain backward compatibility for public APIs where practical.
- Add tests alongside behavior changes.

## References

- `ARCHITECTURE.md`
- `CONTRIBUTING.md`
- `core/src/builtins/README.md`
- `core/tests/README.md`
- `core/tests/CONFORMANCE_TESTS.md`
- `docs/README.md`
- V8 Embedder's Guide: <https://v8.dev/docs/embed>
- Rust V8 bindings: <https://docs.rs/v8/>