# Contributing to jstime

Thank you for considering contributing to jstime! This guide will help you get started.

## Quick Links

* [Architecture Guide](./ARCHITECTURE.md) - Understand how jstime is structured
* [Code of Conduct](#code-of-conduct)
* [Getting Started](#getting-started)
* [Development Workflow](#development-workflow)
* [Pull Requests](#pull-requests)

## Getting Started

### Prerequisites

- **Rust** (stable): Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository

### Setup

```bash
# Clone and build
git clone https://github.com/jstime/jstime.git
cd jstime
cargo build

# Run tests
cargo test

# Try the REPL
cargo run
```

## Project Structure

```
jstime/
├── core/           # Core runtime library (jstime_core)
│   ├── src/
│   │   ├── builtins/    # Built-in JavaScript APIs
│   │   ├── event_loop.rs
│   │   └── lib.rs
│   └── tests/      # Integration tests
├── cli/            # CLI tool
├── docs/           # Documentation
└── examples/       # Example scripts
```

## Development Workflow

### Making Changes

```bash
# Make your changes, then:
cargo fmt --all                        # Format code
cargo clippy --all-targets -- -D warnings  # Check lints (including tests)
cargo test                             # Run tests
```

### Adding Built-in APIs

See [core/src/builtins/README.md](./core/src/builtins/README.md) for detailed instructions on adding new JavaScript APIs.

**Quick overview:**
1. Create `your_api_impl.rs` (Rust implementation)
2. Create `your_api.js` (JavaScript polyfill)
3. Register in `builtins/mod.rs`
4. Write tests in `core/tests/`
5. Update `docs/FEATURES.md`

### Testing

```bash
cargo test                    # All tests
cargo test test_name          # Specific test
cargo test -- --nocapture     # Show output
```

See [core/tests/README.md](./core/tests/README.md) for testing patterns.

## [Code of Conduct](./CODE_OF_CONDUCT.md)

The jstime project has a [Code of Conduct](./CODE_OF_CONDUCT.md). All
individuals participating in the jstime repo and organization will be
expected to abide by the Code of Conduct. Violating the Code of Conduct
will result in action ranging from a conversation about behavior to
being permanently banned from the jstime organization.

### The Spirit of the law

Not all interactions that require remediation are clear violations
of the Code of Conduct. Project maintainers will take appropriate
action, when neccessary, to ensure the jstime community is a space
where individuals can comfortably collaborate and bring their
entire selves. Unfortunately, if bringing your entire self is
infringing on others from doing the same, you may be asked to leave.

## Pull Requests

If you would like to make a change open a Pull Request.

Project maintainers will do their best to review and land code
in a reasonable time frame.

All Pull Requests require CI to be green to land.

It is possible that some Pull Requests may be rejected. The project
maintainers will make best effort to explain why a Pull Request is
rejected in a timely manner.

## Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

* (a) The contribution was created in whole or in part by me and I
  have the right to submit it under the open source license
  indicated in the file; or

* (b) The contribution is based upon previous work that, to the best
  of my knowledge, is covered under an appropriate open source
  license and I have the right under that license to submit that
  work with modifications, whether created in whole or in part
  by me, under the same open source license (unless I am
  permitted to submit under a different license), as indicated
  in the file; or

* (c) The contribution was provided directly to me by some other
  person who certified (a), (b) or (c) and I have not modified
  it.

* (d) I understand and agree that this project and the contribution
  are public and that a record of the contribution (including all
  personal information I submit with it, including my sign-off) is
  maintained indefinitely and may be redistributed consistent with
  this project or the open source license(s) involved.
  
