# jstime Core Crate

The main dependency of this crate is [rusty\_v8](https://github.com/denoland/rusty_v8)
which provides the V8-Rust bindings.

## Features

* **Console API**: Full console implementation
* **Microtask support**: Via `queueMicrotask`
* **URL API**: Built-in URL parsing and manipulation
* **Timers**: setTimeout and setInterval support

## API

```rust
use jstime_core as jstime;

fn main() {
    jstime::init(None);
    let mut scope = jstime::JSTime::new(
        jstime::Options::default()
    );
    scope.run_script("console.log('Hello, World!');", "jstime")
        .expect("ruhroh something went wrong");
}
```
