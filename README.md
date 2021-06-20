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

## Embed it!

Check out the [README.md for jstime-core](./core/README.md) for
instructions on how to embed jstime in your rust application!

## Current Project Team Members

for information about the governance of the jstime project, see
[GOVERNANCE.md](./GOVERNANCE.md).

### Chair

* [MylesBorins](https://github.com/MylesBorins) - **Myles Borins** (he/him)

### Collaborators

* [bengl](https://github.com/bengl) - **Bryan English** (he/him)
* [bdougie](https://github.com/bdougie) - **Brian Douglas**
* [codebytere](https://github.com/codebytere) - **Shelley Vohr**
* [devsnek](https://github.com/devsnek) - **Gus Caplan**
* [EstebanBorai](https://github.com/EstebanBorai) - **Esteban Borai**
* [jalafel](https://github.com/jalafel) - **Jess Tran**
* [MylesBorins](https://github.com/MylesBorins) - **Myles Borins** (he/him)
* [solumos](https://github.com/solumos) - **Tom Hadley**

## TODO

* Finish implementing [console](https://console.spec.whatwg.org/)
* [fetch](https://fetch.spec.whatwg.org/)
* [ESM](https://www.ecma-international.org/ecma-262/11.0/index.html#sec-modules)
* Some sort of system interface, maybe [WASI](https://wasi.dev/)
