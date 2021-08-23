use std::{
	collections::BTreeMap,
	borrow::Borrow
};
use crate::{
	Collection,
	Keyed,
	CollectionRef,
	CollectionMut,
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

impl<K, V> CollectionRef for BTreeMap<K, V> {
	type ItemRef<'a> where Self: 'a = &'a V;
}

impl<K, V> CollectionMut for BTreeMap<K, V> {
	type ItemMut<'a> where Self: 'a = &'a mut V;
}

impl<K, V> Keyed for BTreeMap<K, V> {
	type Key = K;
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

impl<K: Ord, V> MapInsert<K> for BTreeMap<K, V> {
	type Output<'a> where Self: 'a = Option<V>;

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