use std::{
	collections::HashMap,
	borrow::Borrow,
	hash::Hash
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

impl<K, V> Collection for HashMap<K, V> {
	type Item = V;
}

impl<K, V> CollectionRef for HashMap<K, V> {
	type ItemRef<'a> where Self: 'a = &'a V;
}

impl<K, V> CollectionMut for HashMap<K, V> {
	type ItemMut<'a> where Self: 'a = &'a mut V;
}

impl<K, V> Keyed for HashMap<K, V> {
	type Key = K;
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

impl<'a, Q, K: Hash + Eq, V> GetMut<&'a Q> for HashMap<K, V> where K: Borrow<Q>, Q: Hash + Eq + ?Sized {
	#[inline(always)]
	fn get_mut(&mut self, key: &'a Q) -> Option<&mut V> {
		self.get_mut(key)
	}
}

impl<K: Hash + Eq, V> MapInsert<K> for HashMap<K, V> {
	type Output<'a> where Self: 'a = Option<V>;

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