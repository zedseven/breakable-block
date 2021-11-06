//! A shim library for a stable implementation of what is proposed in [RFC 2046](https://rust-lang.github.io/rfcs/2046-label-break-value.html),
//! allowing for short-circuit control flow without needing to return in a
//! function or break in a loop.
//!
//! When the RFC is stabilized, this crate will be deprecated.
//! If you don't need to work on `stable`, you can use
//! `#![feature(label_break_value)]` in your crate root to enable the
//! functionality instead.
//!
//! This crate has no dependencies.
//!
//! ## Example
//! Here's an example, [lifted straight from the RFC documentation](https://rust-lang.github.io/rfcs/2046-label-break-value.html#motivation)
//! with only minor modifications:
//!
//! ```rust
//! use breakable_block::breakable;
//!
//! # fn do_thing() {}
//! # fn do_next_thing() {}
//! # fn do_last_thing() {}
//! # fn condition_not_met() -> bool { true }
//! breakable!('block: {
//! 	do_thing();
//!     if condition_not_met() {
//!         break 'block;
//!     }
//!     do_next_thing();
//!     if condition_not_met() {
//!         break 'block;
//!     }
//!     do_last_thing();
//! });
//! ```

// No std dependency
#![no_std]

/// A breakable block, similar to [RFC 2046](https://rust-lang.github.io/rfcs/2046-label-break-value.html).
///
/// See the crate-level documentation for more information.
#[macro_export]
macro_rules! breakable {
	($label:lifetime: $block:block) => {
		#[allow(clippy::never_loop)]
		$label: loop {
			break {
				$block
			}
		}
	};
}
