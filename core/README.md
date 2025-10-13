# jstime Core Crate

The main dependency of this crate is [rusty\_v8](https://github.com/denoland/rusty_v8)
which provides the V8-Rust bindings.

## Features

* **Temporal API**: Built-in support for the [TC39 Temporal proposal](https://tc39.es/proposal-temporal/) via [temporal_rs](https://github.com/boa-dev/temporal)
* **Console API**: Full console implementation
* **Microtask support**: Via `queueMicrotask`

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

## Using Temporal API

The Temporal API is available globally in all scripts:

```rust
use jstime_core as jstime;

fn main() {
    jstime::init(None);
    let mut scope = jstime::JSTime::new(
        jstime::Options::default()
    );
    
    // Use Temporal API
    scope.run_script(r#"
        const date = new Temporal.PlainDate(2025, 10, 13);
        console.log('Date:', date.year, date.month, date.day);
        
        const now = Temporal.Now.instant();
        console.log('Current instant:', now.epochNanoseconds);
    "#, "jstime").expect("script failed");
}
```
