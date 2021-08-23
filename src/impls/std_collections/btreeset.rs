use std::{
	collections::BTreeSet,
	borrow::Borrow
};
use crate::{
	Collection,
	CollectionRef,
	CollectionMut,
	Len,
	Get,
	Insert,
	Remove,
	Clear
};

impl<T> Collection for BTreeSet<T> {
	type Item = T;
}

impl<T> CollectionRef for BTreeSet<T> {
	type ItemRef<'a> where Self: 'a = &'a T;
}

impl<T> CollectionMut for BTreeSet<T> {
	type ItemMut<'a> where Self: 'a = &'a mut T;
}

impl<T> Len for BTreeSet<T> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<'a, Q, T: Ord> Get<&'a Q> for BTreeSet<T> where T: Borrow<Q>, Q: Ord + ?Sized {
	#[inline(always)]
	fn get(&self, t: &'a Q) -> Option<&T> {
		self.get(t)
	}
}

impl<T: Ord> Insert for BTreeSet<T> {
	type Output<'a> where Self: 'a = bool;

	#[inline(always)]
	fn insert(&mut self, t: T) -> bool {
		self.insert(t)
	}
}

impl<'a, Q, T: Ord> Remove<&'a Q> for BTreeSet<T> where T: Borrow<Q>, Q: Ord + ?Sized {
	#[inline(always)]
	fn remove(&mut self, t: &'a Q) -> Option<T> {
		self.take(t)
	}
}

impl<T: Ord> Clear for BTreeSet<T> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}