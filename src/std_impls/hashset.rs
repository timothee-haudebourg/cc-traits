use std::{
	collections::HashSet,
	borrow::Borrow,
	hash::Hash
};
use crate::{
	Collection,
	Len,
	Insert,
	Remove,
	Clear
};

impl<T> Collection for HashSet<T> {
	type Item = T;
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

impl<'a, T: Hash + Eq> Insert for HashSet<T> {
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