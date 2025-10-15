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

## Features

* **Temporal API**: jstime includes support for the [TC39 Temporal proposal](https://tc39.es/proposal-temporal/), powered by [temporal_rs](https://github.com/boa-dev/temporal). The Temporal API provides modern date and time handling with calendar and timezone awareness.

```javascript
// Get current instant
const now = Temporal.Now.instant();

// Create a plain date
const date = new Temporal.PlainDate(2025, 10, 13);
console.log(date.year, date.month, date.day); // 2025 10 13

// Create a plain time
const time = new Temporal.PlainTime(15, 30, 45);
console.log(time.hour, time.minute, time.second); // 15 30 45

// Create a datetime
const dt = new Temporal.PlainDateTime(2025, 10, 13, 15, 30, 45);

// Parse instant from string
const instant = Temporal.Instant.from('2025-10-13T12:00:00Z');
```

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

* [fetch](https://fetch.spec.whatwg.org/)
* Some sort of system interface, maybe [WASI](https://wasi.dev/)
