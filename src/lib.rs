#![feature(trait_alias)]
use std::ops::{
	Index,
	IndexMut
};

mod std_impls;

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

/// Collection with mutable capacity.
pub trait CapacityMut = Capacity + Reserve;

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

/// Immutable stack data structure.
/// 
/// A stack provides two main operations:
///   - [`PushBack::push_back`], which adds an element to the collection, and
///   - [`PopBack::pop_back`], which removes the most recently added element that was not yet removed.
/// 
/// This trait alias describes the immutables operations derived from the two main operation above:
///   - [`Len::len`], returning the number of elements in the stack, and
///   - [`Back::back`], returning a reference to the most recently added element that was not yet removed.
pub trait Stack<T> = Collection<Item=T> + Len + Back;

/// Mutable stack data structure.
/// 
/// This trait alias describes the mutables operations on a stack.
/// See [`Stack`] for more details.
pub trait StackMut<T> = Stack<T> + BackMut + PushBack + PopBack;

/// Immutable array data structure (conventionally nammed "Vec").
/// 
/// A Vec is essentially a [`Stack`] indexable by a `usize`.
pub trait Vec<T> = Stack<T> + Index<usize, Output=T>;

/// Mutable Vec data structure.
/// 
/// This trait alias describes the mutables operations on a Vec.
/// See [`Vec`] for more details.
pub trait VecMut<T> = Vec<T> + StackMut<T> + IndexMut<usize>;

/// Immutable double-ended queue.
/// 
/// A double-ended queue (abbreviated to deque) is a generalization of a stack in which
/// elements can be added and removed from both ends.
/// Such a data structure provides two additional operations compared to regular stacks:
///   - [`PushFront::push_front`], which adds an element to the front of collection, and
///   - [`PopFront::pop_front`], which removes the front element of the collection.
/// 
/// This trait alias describes the immutables operations available on deques.
pub trait Deque<T> = Stack<T> + Front;

/// Mutable double-ended queue.
/// 
/// This trait alias describes the mutables operations on a deque.
/// See [`Deque`] for more details.
pub trait DequeMut<T> = StackMut<T> + FrontMut + PushFront + PopFront;

/// Immutable indexable deque.
/// 
/// See [`Deque`] and [`Vec`] for more details.
pub trait VecDeque<T> = Deque<T> + Vec<T>;

/// Mutable indexable deque.
/// 
/// See [`VecDeque`], [`DequeMut`] and [`VecMut`] for more details.
pub trait VecDequeMut<T> = VecDeque<T> + DequeMut<T> + VecMut<T>;

/// Imutable set data structure.
/// 
/// A set is an unordered collection storing at most one single copy of each element.
pub trait Set<T> = Len + for<'a> Get<&'a T>;

/// Mutable set data structure.
pub trait SetMut<T> = Set<T> + Insert<Output=bool> + for<'a> Remove<&'a T>;

/// Imutable map data structure.
/// 
/// A map is an unordered collection storing key-value pairs, indexed by the key.
pub trait Map<K, V> = Collection<Item=(K, V)> + Len + for<'a> Get<&'a K>;

/// Mutable map data structure.
pub trait MapMut<K, V> = Map<K, V> + for<'a> GetMut<&'a K>;

/// Imutable slab data structure.
/// 
/// A slab is a linear collection storing each element at a given index.
/// The index of the element is allocated and returned upon insertion.
pub trait Slab<T> = Len + Get<usize, Item=T>;

/// Mutable slab data structure.
pub trait SlabMut<T> = Slab<T> + Insert<Output=usize> + Remove<usize>;