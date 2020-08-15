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

## FYI

This is an extremely basic wrapper around V8. Don't expect
any fancy APIs just yet.

## TODO

* [console](https://console.spec.whatwg.org/)
* [fetch](https://fetch.spec.whatwg.org/)
* [ESM](https://www.ecma-international.org/ecma-262/11.0/index.html#sec-modules)
* Some sort of system interface, maybe [WASI](https://wasi.dev/)
