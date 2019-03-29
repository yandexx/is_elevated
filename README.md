[![Crates.io](https://img.shields.io/crates/v/is_elevated.svg)](https://crates.io/crates/is_elevated)
[![docs.rs](https://docs.rs/is_elevated/badge.svg)](https://docs.rs/is_elevated/)
[![Build status](https://ci.appveyor.com/api/projects/status/6s73oi5piirif047?svg=true)](https://ci.appveyor.com/project/yandexx/is-elevated)

**is_elevated** is a simple Windows-only crate that lets you determine
whether the current process is running as elevated (also known “as
administrator,” or integrity level High), or not (integrity level Medium
or lower).

## Example
```rust
use is_elevated::is_elevated;

if !is_elevated() {
    println!(
        "Warning: the program isn’t running as elevated; some functionality may not work."
    );
}
```
