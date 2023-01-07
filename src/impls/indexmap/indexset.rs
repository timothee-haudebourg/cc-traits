use crate::{
	Capacity, Clear, Collection, CollectionMut, CollectionRef, Get, Insert, Iter, Len, PopBack,
	Remove, Reserve, SimpleCollectionMut, SimpleCollectionRef, WithCapacity,
};
use indexmap::IndexSet;
use std::{borrow::Borrow, hash::Hash};

impl<T> Collection for IndexSet<T> {
	type Item = T;
}

impl<T> CollectionRef for IndexSet<T> {
	type ItemRef<'a> = &'a T where Self: 'a;

	crate::covariant_item_ref!();
}

impl<T> CollectionMut for IndexSet<T> {
	type ItemMut<'a> = &'a mut T where Self: 'a;

	crate::covariant_item_mut!();
}

impl<T> SimpleCollectionRef for IndexSet<T> {
	fn into_ref<'a>(r: &'a T) -> &'a T
	where
		Self: 'a,
	{
		r
	}
}

impl<T> SimpleCollectionMut for IndexSet<T> {
	fn into_mut<'a>(r: &'a mut T) -> &'a mut T
	where
		Self: 'a,
	{
		r
	}
}

impl<T> Len for IndexSet<T> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<'a, Q, T: Hash + Eq> Get<&'a Q> for IndexSet<T>
where
	T: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	fn get(&self, value: &'a Q) -> Option<&T> {
		self.get(value)
	}
}

impl<T: Hash + Eq> Insert for IndexSet<T> {
	type Output = bool;

	#[inline(always)]
	fn insert(&mut self, t: T) -> bool {
		self.insert(t)
	}
}

impl<'a, Q, T: Hash + Eq> Remove<&'a Q> for IndexSet<T>
where
	T: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn remove(&mut self, t: &'a Q) -> Option<T> {
		self.take(t)
	}
}

impl<T: Hash + Eq> Clear for IndexSet<T> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<T> Iter for IndexSet<T> {
	type Iter<'a> = indexmap::set::Iter<'a, T> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<T> Capacity for IndexSet<T> {
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

impl<T: Eq + Hash> Reserve for IndexSet<T> {
	#[inline(always)]
	fn reserve(&mut self, additional: usize) {
		IndexSet::reserve(self, additional)
	}
}

impl<T: Eq + Hash> PopBack for IndexSet<T> {
	#[inline(always)]
	fn pop_back(&mut self) -> Option<T> {
		self.pop()
	}
}

impl<T> Get<usize> for IndexSet<T> {
	#[inline(always)]
	fn get(&self, index: usize) -> Option<&T> {
		self.get_index(index)
	}
}
