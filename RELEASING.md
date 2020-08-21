# Making a release of jstime

This project uses cargo-release to automate cutting,
tagging, and pushing a new release. Publishing to crates.io
is automated by a github action.

## 1. Install cargo-release

```bash
$ cargo install cargo-release
```

## 2. run cargo-release

```bash
$ cargo release <semver>
```
