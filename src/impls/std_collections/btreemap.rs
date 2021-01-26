use std::{
	collections::BTreeMap,
	borrow::Borrow
};
use crate::{
	Collection,
	Len,
	Get,
	GetMut,
	MapInsert,
	Remove,
	Clear
};

impl<K, V> Collection for BTreeMap<K, V> {
	type Item = V;
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

impl<'a, Q, K: Ord, V> Get<&'a Q> for BTreeMap<K, V> where K: Borrow<Q>, Q: Ord + ?Sized {
	#[inline(always)]
	fn get(&self, key: &'a Q) -> Option<&V> {
		self.get(key)
	}
}

impl<'a, Q, K: Ord, V> GetMut<&'a Q> for BTreeMap<K, V> where K: Borrow<Q>, Q: Ord + ?Sized {
	#[inline(always)]
	fn get_mut(&mut self, key: &'a Q) -> Option<&mut V> {
		self.get_mut(key)
	}
}

impl<'a, K: Ord, V> MapInsert<K> for BTreeMap<K, V> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: V) -> Option<V> {
		self.insert(key, value)
	}
}

impl<'a, Q, K: Ord, V> Remove<&'a Q> for BTreeMap<K, V> where K: Borrow<Q>, Q: Ord + ?Sized {
	#[inline(always)]
	fn remove(&mut self, key: &'a Q) -> Option<V> {
		self.remove(key)
	}
}

impl<K: Ord, V> Clear for BTreeMap<K, V> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}