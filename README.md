# dynamorio-sys

[![crates.io](https://img.shields.io/crates/v/dynamorio-sys.svg)](https://crates.io/crates/dynamorio-sys)
[![mio](https://docs.rs/dynamorio-sys/badge.svg)](https://docs.rs/dynamorio-sys/)
![Lines of Code](https://tokei.rs/b1/github/elast0ny/dynamorio-sys)

A crate using bindgen to automatically generate Rust bindings to [DynamoRIO](https://dynamorio.org).
For safe Rust bindings to the DynamoRIO dynamic binary instrumentation framework, you may want to consider using [dynamorio-rs](https://github.com/StephanvanSchaik/dynamorio-rs) instead.
[dynamorio-rs](https://github.com/StephanvanSchaik/dynamorio-rs) uses this crate to provide safe Rust bindings to DynamoRIO such that you can write DynamoRIO clients in Rust.

# Supported Platforms

dynamorio-sys is currently available for the following platforms:

- [x] Microsoft Windows
- [x] Linux

# Usage

To build the project:

```bash
git clone --recurse-submodules https://github.com/elast0ny/dynamorio-sys.git
cd dynamorio-sys
cargo build
```

In order to enable extensions, use their respective names as features for the crate e.g :

```toml
dynamorio-sys = {version = "*", features = ["mgr", "x", "reg", "wrap", "syms"]}
```

## Version
The crate's major and minor version match the major/minor version of the built DynamoRIO. This should allow users to use the latest DynamoRIO patch for their current major/minor while also allowing this crate to fix build issues and update its own patch version.

