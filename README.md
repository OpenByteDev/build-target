# build-target

[![Build](https://github.com/OpenByteDev/build-target/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/OpenByteDev/build-target/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/build-target.svg)](https://crates.io/crates/build-target)
[![Documentation](https://docs.rs/build-target/badge.svg)](https://docs.rs/build-target)
[![dependency status](https://deps.rs/repo/github/openbytedev/build-target/status.svg)](https://deps.rs/repo/github/openbytedev/build-target)
[![MIT](https://img.shields.io/crates/l/build-target.svg)](https://github.com/OpenByteDev/build-target/blob/master/LICENSE)

A crate that provides programmatic access to information about the current build target inside `build.rs`.

## Example

```rust
// inside build.rs
fn main() {
	// panics are just the easiest way to output in build scripts.
    panic!("current build target (arch={}, env={}, os={}, family={})",
        Arch::target().unwrap(),
        Env::target().unwrap(),
        Os::target().unwrap(),
        Family::target().unwrap()
    );
}
```

## Attribution
This crate is inspired from `platforms` and is based on it's code.

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/build-target/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
