# dynamorio-sys

[![crates.io](https://img.shields.io/crates/v/dynamorio-sys.svg)](https://crates.io/crates/dynamorio-sys)
[![mio](https://docs.rs/dynamorio-sys/badge.svg)](https://docs.rs/dynamorio-sys/)
![Lines of Code](https://tokei.rs/b1/github/elast0ny/dynamorio-sys)


A crate to automatically generate Rust bindings to DynamoRIO.


__DISCLAIMER__ : I built this crate to facilitate work I was doing on another project. I am willing to do minimal maintenance if issues arise and/or give  ownership to a more motivated developer.

# Usage

In order to enable extensions, use their respective names as features for the crate e.g :
```toml
dynamorio-sys = {version = "*", features = ["mgr", "x", "reg", "wrap", "syms"]}
```

## Version
The crate's major and minor version match the major/minor version of the built DynamoRIO. This should allow users to use the latest DynamoRIO patch for their current major/minor while also allowing this crate to fix build issues and update its own patch version.

