use std::{
	collections::HashMap,
	borrow::Borrow,
	hash::Hash
};
use crate::{
	Collection,
	Len,
	Get,
	MapInsert,
	Remove,
	Clear
};

impl<K, V> Collection for HashMap<K, V> {
	type Item = V;
}

impl<K, V> Len for HashMap<K, V> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<'a, Q, K: Hash + Eq, V> Get<&'a Q> for HashMap<K, V> where K: Borrow<Q>, Q: Hash + Eq + ?Sized {
	#[inline(always)]
	fn get(&self, key: &'a Q) -> Option<&V> {
		self.get(key)
	}
}

impl<'a, K: Hash + Eq, V> MapInsert<K> for HashMap<K, V> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: V) -> Option<V> {
		self.insert(key, value)
	}
}

impl<'a, Q, K: Hash + Eq, V> Remove<&'a Q> for HashMap<K, V> where K: Borrow<Q>, Q: Hash + Eq + ?Sized {
	#[inline(always)]
	fn remove(&mut self, key: &'a Q) -> Option<V> {
		self.remove(key)
	}
}

impl<K, V> Clear for HashMap<K, V> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}