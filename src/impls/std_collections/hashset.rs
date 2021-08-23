use std::{
	collections::HashSet,
	borrow::Borrow,
	hash::Hash
};
use crate::{
	Collection,
	CollectionRef,
	CollectionMut,
	Len,
	Get,
	Insert,
	Remove,
	Clear,
	Iter
};

impl<T> Collection for HashSet<T> {
	type Item = T;
}

impl<T> CollectionRef for HashSet<T> {
	type ItemRef<'a> where Self: 'a = &'a T;
}

impl<T> CollectionMut for HashSet<T> {
	type ItemMut<'a> where Self: 'a = &'a mut T;
}

impl<T> Len for HashSet<T> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<'a, Q, T: Hash + Eq> Get<&'a Q> for HashSet<T> where T: Borrow<Q>, Q: Hash + Eq + ?Sized {
	fn get(&self, value: &'a Q) -> Option<&T> {
		self.get(value)
	}
}

impl<T: Hash + Eq> Insert for HashSet<T> {
	type Output = bool;

	#[inline(always)]
	fn insert(&mut self, t: T) -> bool {
		self.insert(t)
	}
}

impl<'a, Q, T: Hash + Eq> Remove<&'a Q> for HashSet<T> where T: Borrow<Q>, Q: Hash + Eq + ?Sized {
	#[inline(always)]
	fn remove(&mut self, t: &'a Q) -> Option<T> {
		self.take(t)
	}
}

impl<T: Hash + Eq> Clear for HashSet<T> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<T> Iter for HashSet<T> {
	type Iter<'a> where T: 'a = std::collections::hash_set::Iter<'a, T>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}