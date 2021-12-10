//! This crate provide traits to describe common operations available on data structures.
//! This is particularly useful when building new types on top of generic data structures without relying on the actual implementation of the underlying data structure.
//!
//! Here is an example of the kind of traits provided by this crate:
//! ```rust
//! # use cc_traits::Collection;
//! /// Mutable collection where new elements can be inserted.
//! pub trait Insert: Collection {
//!     /// The output of the insertion function.
//!     type Output;
//!
//!     /// Insert a new element in the collection.
//!     fn insert(&mut self, element: Self::Item) -> Self::Output;
//! }
//! ```
//!
//! # Usage
//!
//! Such traits can be used to define collections with special properties,
//! independently of the actual internal data structure.
//! For instance the following code defines an `Ordered<S>` stack collection,
//! guarantying the well-sortedness of the elements in the stack.
//!
//! ```rust
//! use cc_traits::{
//!     Collection,
//!     Back,
//!     PushBack
//! };
//!
//! /// Ordered stack.
//! pub struct Ordered<S> {
//!     inner: S
//! }
//!
//! impl<S> Ordered<S> {
//!     pub fn new() -> Self where S: Default {
//!         Ordered {
//!             inner: S::default()
//!         }
//!     }
//! }
//!
//! impl<S> Ordered<S> {
//!     /// Push the given element on the stack iff it is grater or equal
//!     /// to every other element already in the stack.
//!     pub fn try_push<T>(&mut self, element: T) -> Result<(), T>
//!     where
//!         T: PartialOrd,
//!         S: Collection<Item=T> + Back + PushBack, // `S` must be a stack providing `back` and `push_back`.
//!         for<'a> S::ItemRef<'a>: PartialOrd<&'a T> // The reference type must be comparable with other reference types.
//!     {
//!         if self.inner.back().map(|back| back <= &element).unwrap_or(true) {
//!             self.inner.push_back(element);
//!             Ok(())
//!         } else {
//!             Err(element)
//!         }
//!     }
//! }
//!
//! let mut vec: Ordered<Vec<i32>> = Ordered::new(); // a `Vec` is a stack so it works.
//!
//! assert!(vec.try_push(1).is_ok());
//! assert!(vec.try_push(2).is_ok());
//! assert!(vec.try_push(0).is_err());
//!
//! use std::collections::VecDeque;
//! let mut deque: Ordered<VecDeque<i32>> = Ordered::new(); // a `VecDeque` is also a stack.
//!
//! assert!(deque.try_push(1).is_ok());
//! assert!(deque.try_push(2).is_ok());
//! assert!(deque.try_push(0).is_err());
//! ```
//!
//! # Trait aliases
//!
//! By enabling the `nightly` you can get access to
//! some trait alias definitions that can be useful to reduce the
//! verbosity of your code.
//! Here is an example of such aliases defining the common interface of stacks:
//! ```ignore
//! pub trait Stack<T> = Collection<Item=T> + Len + Back;
//! pub trait StackMut<T> = Stack<T> + BackMut + PushBack + PopBack;
//! ```
//!
//! # Standard library
//!
//! By default, all the traits defined in this crate are implemented (when relevent)
//! for the standard library collections.
//! You can disable it by using the `nostd` feature.
//!
//! # Foreign implementations
//!
//! In addition to the standard library,
//! traits are implemented for
//! some popular crates if you enable the feature of the same name.
//! Here are the supported crates:
//!
//!   - [`slab`](https://crates.io/crates/slab) providing the `Slab` collection.
//!   - [`smallvec`](https://crates.io/crates/smallvec) providing the `SmallVec` collection.
//!   - [`serde_json`](https://crates.io/crates/serde_json) providing the `Map<String, Value>` collection for JSON objects.
//!   - [`ijson`](https://crates.io/crates/ijson) providing the `IObject` and `IArray` collections.
#![feature(generic_associated_types)]
#![cfg_attr(feature = "nightly", feature(trait_alias))]
#![cfg_attr(feature = "raw-api", feature(hash_raw_entry))]

mod impls;
mod macros;

#[cfg(feature = "nightly")]
mod alias;

mod entry_api;
pub use entry_api::*;

#[cfg(feature = "nightly")]
pub use alias::*;

use std::ops::{Deref, DerefMut};

/// Abstract collection.
pub trait Collection {
	/// Type of the items of the collection.
	type Item;
}

/// Abstract collection that can be immutably referenced.
pub trait CollectionRef: Collection {
	/// Type of references to items of the collection.
	type ItemRef<'a>: Clone + Deref<Target = Self::Item>
	where
		Self: 'a;

	/// Changes an item reference into a shorter lived reference.
	///
	/// Item references are [covariant](https://doc.rust-lang.org/nomicon/subtyping.html#variance) with
	/// regard to the defined lifetime parameter `'a`.
	/// Since this cannot be directly expressed by the type system, this associated function
	/// allows one to explicitly shorten the reference's lifetime.
	///
	/// You can use the [`covariant_item_ref!`] macro to automatically
	/// implement this function.
	fn upcast_item_ref<'short, 'long: 'short>(r: Self::ItemRef<'long>) -> Self::ItemRef<'short>
	where
		Self: 'long;
}

/// Abstract collection that can be mutably referenced.
pub trait CollectionMut: Collection {
	/// Type of mutable references to items of the collection.
	type ItemMut<'a>: DerefMut<Target = Self::Item>
	where
		Self: 'a;

	/// Changes an item mutable reference into a shorter lived mutable reference.
	///
	/// See the [`CollectionRef::upcast_item_ref`] function for more information.
	/// You can use the [`covariant_item_mut!`] macro to automatically
	/// implement this function.
	fn upcast_item_mut<'short, 'long: 'short>(r: Self::ItemMut<'long>) -> Self::ItemMut<'short>
	where
		Self: 'long;
}

/// Abstract keyed collection.
pub trait Keyed: Collection {
	/// Type of the keys indexing each item of the collection.
	type Key;
}

/// Abstract keyed collection whose key can be referenced.
pub trait KeyedRef: Keyed {
	/// Type of references to keys of the collection.
	type KeyRef<'a>: Clone + Deref<Target = Self::Key>
	where
		Self: 'a;

	/// Changes a key reference into a shorter lived reference.
	///
	/// See the [`CollectionRef::upcast_item_ref`] function for more information.
	/// You can use the [`covariant_key_ref!`] macro to automatically
	/// implement this function.
	fn upcast_key_ref<'short, 'long: 'short>(r: Self::KeyRef<'long>) -> Self::KeyRef<'short>
	where
		Self: 'long;
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
pub trait Get<T>: CollectionRef {
	/// Returns a reference to the item stored behind the given key (if any).
	fn get(&self, key: T) -> Option<Self::ItemRef<'_>>;

	/// Checks if the collection contains an item behind the given key.
	fn contains_key(&self, key: T) -> bool {
		self.get(key).is_some()
	}

	/// Checks if the collection contains an item behind the given key.
	#[deprecated = "use `contains_key`"]
	fn contains(&self, key: T) -> bool {
		self.get(key).is_some()
	}
}

/// Mutably queryable collection.
pub trait GetMut<T>: Get<T> + CollectionMut {
	/// Returns a mutable reference to the item stored behind the given key (if any).
	fn get_mut(&mut self, key: T) -> Option<Self::ItemMut<'_>>;
}

/// Queryable map.
pub trait GetKeyValue<T>: CollectionRef + KeyedRef {
	/// Returns the key-value pair matching the given `key`.
	fn get_key_value(&self, key: T) -> Option<(Self::KeyRef<'_>, Self::ItemRef<'_>)>;
}

/// Mutably queryable map.
pub trait GetKeyValueMut<T>: CollectionMut + KeyedRef {
	/// Returns the key-value pair matching the given `key`, with a mutable reference to the value.
	fn get_key_value_mut(&mut self, key: T) -> Option<(Self::KeyRef<'_>, Self::ItemMut<'_>)>;
}

/// Collection exposing a reference to its front element.
pub trait Front: CollectionRef {
	/// Get a reference to the front element of the collection.
	fn front(&self) -> Option<Self::ItemRef<'_>>;
}

impl<T: Get<usize> + Len> Front for T {
	fn front(&self) -> Option<Self::ItemRef<'_>> {
		match self.len() {
			0 => None,
			_ => self.get(0),
		}
	}
}

/// Collection exposing a reference to its back element.
pub trait Back: CollectionRef {
	/// Get a reference to the back element of the collection.
	fn back(&self) -> Option<Self::ItemRef<'_>>;
}

impl<T: Get<usize> + Len> Back for T {
	fn back(&self) -> Option<Self::ItemRef<'_>> {
		match self.len() {
			0 => None,
			l => self.get(l - 1),
		}
	}
}

/// Collection exposing a mutable reference to its front element.
pub trait FrontMut: CollectionMut {
	/// Get a mutable reference to the front element of the collection.
	fn front_mut(&mut self) -> Option<Self::ItemMut<'_>>;
}

impl<T: GetMut<usize> + Len> FrontMut for T {
	fn front_mut(&mut self) -> Option<Self::ItemMut<'_>> {
		match self.len() {
			0 => None,
			_ => self.get_mut(0),
		}
	}
}

/// Collection exposing a mutable reference to its back element.
pub trait BackMut: CollectionMut {
	/// Get a mutable reference to the back element of the collection.
	fn back_mut(&mut self) -> Option<Self::ItemMut<'_>>;
}

impl<T: GetMut<usize> + Len> BackMut for T {
	fn back_mut(&mut self) -> Option<Self::ItemMut<'_>> {
		match self.len() {
			0 => None,
			l => self.get_mut(l - 1),
		}
	}
}

/// Mutable collection where new elements can be inserted.
pub trait Insert: Collection {
	/// The output of the insertion function.
	type Output;

	/// Insert a new element in the collection.
	fn insert(&mut self, element: Self::Item) -> Self::Output;
}

/// Mutable map where new new key-value pairs can be inserted.
pub trait MapInsert<K>: Collection {
	/// The output of the insertion function.
	type Output;

	/// Insert a new key-value pair in the collection.
	fn insert(&mut self, key: K, value: Self::Item) -> Self::Output;
}

/// Mutable map that supports the entry api
pub trait EntryApi: Keyed {
	type Occ<'a>: OccupiedEntry<'a, K = Self::Key, V = Self::Item>
	where
		Self: 'a;
	type Vac<'a>: KeyVacantEntry<'a, K = Self::Key, V = Self::Item>
	where
		Self: 'a;

	/// Gets the given key's corresponding entry in the map for in-place manipulation.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::{EntryApi, Entry, Get};
	///
	/// fn make_map() -> impl EntryApi<Key=char,Item=u32> + Index<&'static char, Output=u32> + Get<&'static char> { HashMap::new() }
	/// let mut letters = make_map();
	///
	/// for ch in "a short treatise on fungi".chars() {
	///     let counter = letters.entry(ch).or_insert(0);
	///     *counter += 1;
	/// }
	///
	/// assert_eq!(letters[&'s'], 2);
	/// assert_eq!(letters[&'t'], 3);
	/// assert_eq!(letters[&'u'], 1);
	/// assert!(!letters.contains_key(&'y'));
	/// ```
	fn entry(&mut self, key: Self::Key) -> Entry<Self::Occ<'_>, Self::Vac<'_>>;
}

#[cfg(feature = "raw-api")]
pub trait EntryRefApi<Q: ToOwned<Owned = Self::Key>>: Keyed {
	type Occ<'a>: OccupiedEntry<'a, K = Self::Key, V = Self::Item>
	where
		Self: 'a,
		Q: 'a;
	type Vac<'a, 'b: 'a>: VacantEntry<'a, K = Self::Key, V = Self::Item>
	where
		Self: 'a,
		Q: 'a + 'b,
		'a: 'b;

	/// Gets the given key's corresponding entry in the map for in-place manipulation.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::{EntryRefApi, Entry, Get};
	///
	/// fn make_map() -> impl EntryRefApi<&'static str, Key=String,Item=u32> + Index<&'static str, Output=u32> + Get<&'static str> { HashMap::new() }
	/// let mut words = make_map();
	///
	/// for s in "foo bar bar".split(' ') {
	///     let counter = letters.entry_ref(s).or_insert(0);
	///     *counter += 1;
	/// }
	///
	/// assert_eq!(words["bar"], 2);
	/// assert_eq!(words["foo"], 1);
	/// assert!(!words.contains_key("baz"));
	/// ```
	fn entry_ref<'a, 'b: 'a>(&'a mut self, key: &'b Q) -> Entry<Self::Occ<'a>, Self::Vac<'a, 'b>>;
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
	/// Remove the element identified by the given `key`.
	fn remove(&mut self, key: T) -> Option<Self::Item>;
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

/// Iterable collection.
pub trait Iter: CollectionRef {
	/// Iterator type.
	type Iter<'a>: Iterator<Item = Self::ItemRef<'a>>
	where
		Self: 'a;

	/// Create an iterator over the items of the collection.
	fn iter(&self) -> Self::Iter<'_>;
}

/// Mutably iterable collection.
pub trait IterMut: CollectionMut {
	/// Iterator type.
	type IterMut<'a>: Iterator<Item = Self::ItemMut<'a>>
	where
		Self: 'a;

	/// Create an iterator over the mutable items of the collection.
	fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

pub trait MapIter: KeyedRef + CollectionRef {
	type Iter<'a>: Iterator<Item = (Self::KeyRef<'a>, Self::ItemRef<'a>)>
	where
		Self: 'a;

	fn iter(&self) -> Self::Iter<'_>;
}

pub trait MapIterMut: KeyedRef + CollectionMut {
	type IterMut<'a>: Iterator<Item = (Self::KeyRef<'a>, Self::ItemMut<'a>)>
	where
		Self: 'a;

	fn iter_mut(&mut self) -> Self::IterMut<'_>;
}
