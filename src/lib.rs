mod std_impls;

#[cfg(feature="nightly")]
mod alias;

/// Abstract collection.
pub trait Collection {
	/// Type of the items of the collection.
	type Item;
}

/// Collection that can be created with a minimum given capacity.
pub trait WithCapacity {
	/// Creates a new instance of `Self` with the given minimum capacity.
	fn with_capacity(capacity: usize) -> Self;
}

/// Sized collection.
pub trait Len {
	/// Returns the number of elements in the collection.
	fn len(&self) -> usize;

	/// Checks if the collection is empty.
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

/// Collection with known capacity.
pub trait Capacity {
	/// Returns the current capacity of the collection.
	/// 
	/// This corresponds to the number of elements the collection can hold without reallocation.
	fn capacity(&self) -> usize;
}

/// Collection that can extend their capacity.
pub trait Reserve {
	/// Reserve enough memory for `edditional` more elements.
	fn reserve(&mut self, additional: usize);
}

/// Queryable collection.
pub trait Get<T>: Collection {
	/// Returns a reference to the item stored behind the given key (if any).
	fn get(&self, key: T) -> Option<&Self::Item>;

	/// Checks if the collection contains an item behind the given key.
	fn contains(&self, key: T) -> bool {
		self.get(key).is_some()
	}
}

/// Mutably queryable collection.
pub trait GetMut<T>: Get<T> {
	/// Returns a mutable reference to the item stored behind the given key (if any).
	fn get_mut(&mut self, key: T) -> Option<&mut Self::Item>;
}

/// Collection exposing a reference to its front element.
pub trait Front: Collection {
	/// Get a reference to the front element of the collection.
	fn front(&self) -> Option<&Self::Item>;
}

/// Collection exposing a reference to its back element.
pub trait Back: Collection {
	/// Get a reference to the back element of the collection.
	fn back(&self) -> Option<&Self::Item>;
}

/// Collection exposing a mutable reference to its front element.
pub trait FrontMut: Collection {
	/// Get a mutable reference to the front element of the collection.
	fn front_mut(&mut self) -> Option<&mut Self::Item>;
}

/// Collection exposing a mutable reference to its back element.
pub trait BackMut: Collection {
	/// Get a mutable reference to the back element of the collection.
	fn back_mut(&mut self) -> Option<&mut Self::Item>;
}

/// Mutable collection where new elements can be inserted.
pub trait Insert: Collection {
	/// The output of the insertion function.
	type Output;

	/// Insert a new element in the collection.
	fn insert(&mut self, element: Self::Item) -> Self::Output;
}

/// Mutable collection where new elements can be pushed on the front.
pub trait PushFront: Collection {
	/// The output of the push function.
	type Output;

	/// Push a new element on the front of the collection.
	fn push_front(&mut self, element: Self::Item) -> Self::Output;
}

/// Mutable collection where new elements can be pushed on the back.
pub trait PushBack: Collection {
	/// The output of the push function.
	type Output;

	/// Push a new element on the back of the collection.
	fn push_back(&mut self, element: Self::Item) -> Self::Output;
}

/// Mutable collection where elements can be removed from.
pub trait Remove<T>: Collection {
	/// Remove the element identified by the given `index`.
	fn remove(&mut self, index: T) -> Option<Self::Item>;
}

/// Mutable collection where elements can be popped from the front.
pub trait PopFront: Collection {
	/// Remove the front element of the collection and return it (if any).
	fn pop_front(&mut self) -> Option<Self::Item>;
}

/// Mutable collection where elements can be popped from the back.
pub trait PopBack: Collection {
	/// Remove the back element of the collection and return it (if any).
	fn pop_back(&mut self) -> Option<Self::Item>;
}

/// Clearable collection.
pub trait Clear {
	/// Remove all the elements of the collection.
	fn clear(&mut self);
}