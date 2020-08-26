# jstime Core Crate

The main dependency of this crate is [rusty\_v8](https://github.com/denoland/rusty_v8)
which provides the V8-Rust bindings.

## API

```rust
use jstime_core::module;

fn main() {
  module::run("console.log('hello world')");
}
```
