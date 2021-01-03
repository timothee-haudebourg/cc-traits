# Common Collection Traits

<table><tr>
  <td><a href="https://docs.rs/cc-traits">Documentation</a></td>
  <td><a href="https://crates.io/crates/cc-traits">Crate informations</a></td>
  <td><a href="https://github.com/timothee-haudebourg/cc-traits">Repository</a></td>
</tr></table>

This crate provide traits to describe common operations available on data structures.
This is particularly useful when building new types on top of generic data structures without relying on the actual implementation of the underlying data structure.

## Usage

```rust
use cc_traits::{
	Collection,
	Back,
	PushBack
};

/// Ordered stack.
pub struct Ordered<S> {
	inner: S
}

impl<S> Ordered<S> {
	pub fn new() -> Self where S: Default {
		Ordered {
			inner: S::default()
		}
	}
}

impl<S> Ordered<S> {
	/// Push the given element on the stack iff it is grater or equal
	/// to every other element already in the stack.
	pub fn try_push<T>(&mut self, element: T) -> Result<(), T>
	where
		T: PartialOrd,
		S: Collection<Item=T> + Back + PushBack // `S` must be a stack providing `back` and `push_back`.
	{
		if self.inner.back().map(|back| back <= &element).unwrap_or(true) {
			self.inner.push_back(element);
			Ok(())
		} else {
			Err(element)
		}
	}
}

fn main() {
	let mut vec: Ordered<Vec<i32>> = Ordered::new(); // a `Vec` is a stack so it works.

	assert!(vec.try_push(1).is_ok());
	assert!(vec.try_push(2).is_ok());
	assert!(vec.try_push(0).is_err());

	use std::collections::VecDeque;
	let mut deque: Ordered<VecDeque<i32>> = Ordered::new(); // a `VecDeque` is also a stack.

	assert!(deque.try_push(1).is_ok());
	assert!(deque.try_push(2).is_ok());
	assert!(deque.try_push(0).is_err());
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

If the original `egl` crate was licensed only under the Apache 2.0 license,
I believe I have made enough breaking changes so that no relevant code from the
original code remains and the rest can be relicensed.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
