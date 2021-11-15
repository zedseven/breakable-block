# breakable-block
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![# Issues](https://img.shields.io/github/issues/zedseven/breakable-block.svg?logo=github)](https://github.com/zedseven/breakable-block/issues)
[![Crates.io](https://img.shields.io/crates/v/breakable-block.svg?logo=rust)](https://crates.io/crates/breakable-block)
[![Crate Downloads](https://img.shields.io/crates/d/breakable-block.svg?logo=azure-artifacts)](https://crates.io/crates/breakable-block)

A shim library for a stable implementation of what is proposed in
[RFC 2046](https://rust-lang.github.io/rfcs/2046-label-break-value.html),
allowing for short-circuit control flow without needing to return in a
function or break in a loop.

When the RFC is stabilized, this crate will be deprecated. If you don't
need to work on `stable`, you can use
```rust
#![feature(label_break_value)]
```
in your crate root to enable the functionality instead.

This crate has no dependencies.

## How to Use It
Here's an example, [lifted straight from the RFC documentation](https://rust-lang.github.io/rfcs/2046-label-break-value.html#motivation)
with only minor modifications:

```rust
use breakable_block::breakable;

breakable!('block: {
    do_thing();
    if condition_not_met() {
        break 'block;
    }
    do_next_thing();
    if condition_not_met() {
        break 'block;
    }
    do_last_thing();
});
```

The only difference in syntax from the example in the RFC is the wrapping
macro call.

## Project License
This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in *breakable-block* by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
