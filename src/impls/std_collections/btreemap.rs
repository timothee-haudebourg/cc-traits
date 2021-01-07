use std::{
	collections::BTreeMap,
	borrow::Borrow
};
use crate::{
	Collection,
	Len,
	Insert,
	Remove,
	Clear
};

impl<K, V> Collection for BTreeMap<K, V> {
	type Item = (K, V);
}

impl<K, V> Len for BTreeMap<K, V> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<'a, K: Ord, V> Insert for BTreeMap<K, V> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, (key, value): (K, V)) -> Option<V> {
		self.insert(key, value)
	}
}

impl<'a, Q, K: Ord, V> Remove<&'a Q> for BTreeMap<K, V> where K: Borrow<Q>, Q: Ord + ?Sized {
	#[inline(always)]
	fn remove(&mut self, key: &'a Q) -> Option<(K, V)> {
		self.remove_entry(key)
	}
}

impl<K: Ord, V> Clear for BTreeMap<K, V> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}