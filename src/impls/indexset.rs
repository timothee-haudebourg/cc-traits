use crate::{
	Capacity, Clear, Collection, CollectionMut, CollectionRef, Get, Insert, Iter, Len, PopBack,
	Remove, Reserve, SimpleCollectionMut, SimpleCollectionRef, WithCapacity,
};
use core::hash::{BuildHasher, Hash};
use indexmap::{Equivalent, IndexSet};

impl<T, S> Collection for IndexSet<T, S> {
	type Item = T;
}

impl<T, S> CollectionRef for IndexSet<T, S> {
	type ItemRef<'a> = &'a T where Self: 'a;

	crate::covariant_item_ref!();
}

impl<T, S> CollectionMut for IndexSet<T, S> {
	type ItemMut<'a> = &'a mut T where Self: 'a;

	crate::covariant_item_mut!();
}

impl<T, S> SimpleCollectionRef for IndexSet<T, S> {
	fn into_ref<'a>(r: &'a T) -> &'a T
	where
		Self: 'a,
	{
		r
	}
}

impl<T, S> SimpleCollectionMut for IndexSet<T, S> {
	fn into_mut<'a>(r: &'a mut T) -> &'a mut T
	where
		Self: 'a,
	{
		r
	}
}

impl<T, S> Len for IndexSet<T, S> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<'a, Q, T, S: BuildHasher> Get<&'a Q> for IndexSet<T, S>
where
	T: Eq + Hash,
	Q: Hash + Equivalent<T> + ?Sized,
{
	#[inline(always)]
	fn get(&self, key: &'a Q) -> Option<&T> {
		self.get(key)
	}
}

impl<T: Hash + Eq, S: BuildHasher> Insert for IndexSet<T, S> {
	type Output = bool;

	#[inline(always)]
	fn insert(&mut self, item: T) -> Self::Output {
		self.insert(item)
	}
}

/// This implementation uses the `IndexSet::swap_take` function.
impl<'a, Q, T, S> Remove<&'a Q> for IndexSet<T, S>
where
	Q: Hash + Equivalent<T> + ?Sized,
	T: Eq + Hash,
	S: BuildHasher,
{
	#[inline(always)]
	fn remove(&mut self, item: &'a Q) -> Option<T> {
		self.swap_take(item)
	}
}

/// Remove an element based on an index. Uses `IndexSet::swap_remove_index` under the hood.
impl<T, S> Remove<usize> for IndexSet<T, S> {
	#[inline(always)]
	fn remove(&mut self, idx: usize) -> Option<T> {
		self.swap_remove_index(idx)
	}
}

impl<T, S> Clear for IndexSet<T, S> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<T, S> Iter for IndexSet<T, S> {
	type Iter<'a> = indexmap::set::Iter<'a, T> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<T, S> Capacity for IndexSet<T, S> {
	#[inline(always)]
	fn capacity(&self) -> usize {
		self.capacity()
	}
}

impl<T> WithCapacity for IndexSet<T> {
	#[inline(always)]
	fn with_capacity(capacity: usize) -> Self {
		IndexSet::with_capacity(capacity)
	}
}

impl<T: Eq + Hash, S: BuildHasher> Reserve for IndexSet<T, S> {
	#[inline(always)]
	fn reserve(&mut self, additional: usize) {
		IndexSet::reserve(self, additional)
	}
}

impl<T: Eq + Hash, S: BuildHasher> PopBack for IndexSet<T, S> {
	#[inline(always)]
	fn pop_back(&mut self) -> Option<T> {
		self.pop()
	}
}

impl<T, S> Get<usize> for IndexSet<T, S> {
	#[inline(always)]
	fn get(&self, idx: usize) -> Option<&T> {
		self.get_index(idx)
	}
}
