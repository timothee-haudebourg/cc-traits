use std::collections::vec_deque::VecDeque;
use cc_traits::{Back, Collection, PushBack};

/// Ordered stack.
pub struct Ordered<S> {
	inner: S,
}

impl<S> Ordered<S> {
	pub fn new() -> Self
	where
		S: Default,
	{
		Ordered {
			inner: S::default(),
		}
	}
}

impl<S> Ordered<S> {
	/// Push the given element on the stack iff it is grater or equal
	/// to every other element already in the stack.
	pub fn try_push<T>(&mut self, element: T) -> Result<(), T>
	where
		T: PartialOrd,
		S: Collection<Item = T> + Back + PushBack, // `S` must be a stack providing `back` and `push_back`.
		for<'a> S::ItemRef<'a>: PartialOrd<&'a T>, // The reference type must be comparable with other reference types.
	{
		if self
			.inner
			.back()
			.map(|back| back <= &element)
			.unwrap_or(true)
		{
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

	let mut deque: Ordered<VecDeque<i32>> = Ordered::new(); // a `VecDeque` is also a stack.

	assert!(deque.try_push(1).is_ok());
	assert!(deque.try_push(2).is_ok());
	assert!(deque.try_push(0).is_err());
}
