use cc_traits::{Back, Collection, PushBack};
#[cfg(feature = "std")]
use std::collections::VecDeque;

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
	#[cfg(feature = "std")]
	fn ordered_stack_usage<S>()
	where
		S: Default + Collection<Item = i32> + Back + PushBack,
		for<'a> S::ItemRef<'a>: PartialOrd<&'a i32>,
	{
		let mut ordered: Ordered<S> = Ordered::new();
		assert!(ordered.try_push(1).is_ok());
		assert!(ordered.try_push(2).is_ok());
		assert!(ordered.try_push(0).is_err());
	}

	#[cfg(feature = "std")]
	ordered_stack_usage::<Vec<i32>>(); // a `Vec` is a stack so it works.

	#[cfg(feature = "std")]
	ordered_stack_usage::<VecDeque<i32>>(); // a `VecDeque` is also a stack.
}
