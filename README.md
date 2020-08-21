# jstime

> Another JavaScript Runtime

![jstime logo. Kinda looks like shrek](./logo.png)

## Using the binary

You can find the latest jstime binary on the [release page](https://github.com/jstime/jstime/releases)

Alternatively you can install with cargo

```bash
$ cargo install jstime
```

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
