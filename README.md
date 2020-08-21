# jstime

> Another JavaScript Runtime

![jstime logo. Kinda looks like shrek](./logo.png)

## Getting Started

Honestly I'm just learning rust right now 😅

```bash
$ cargo run # run a js repl!
$ cargo run path-to-js.js # run a js file!
$ cargo build --release # make a prod build
```

## Using the jstime binary

### As a repl

```bash
$ jstime

Welcome to jstime!

>>
```

### Run a script

```bash
$ cat hello-world.js
console.log("hello world");

$ jstime hello-world.js
hello world

```
## It's a library too!

```rust
use jstime::script;

fn main() {
  script::run("console.log('hello world')");
}
```

## TODO

* [console](https://console.spec.whatwg.org/)
* [fetch](https://fetch.spec.whatwg.org/)
* [ESM](https://www.ecma-international.org/ecma-262/11.0/index.html#sec-modules)
* Some sort of system interface, maybe [WASI](https://wasi.dev/)
