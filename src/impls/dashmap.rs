use dashmap::{mapref::one::{Ref, RefMut}, DashMap};
use std::{cmp::Eq, borrow::Borrow, hash::{BuildHasher, Hash}};
use crate::{
	Collection,
	CollectionRef,
	CollectionLock,
	Len,
	Get,
	GetLock,
	MapInsertLock,
	RemoveLock,
	ClearLock
};

impl<K, V, S> Collection for DashMap<K, V, S>
		where K: Eq + Hash, S: BuildHasher {
	type Item = V;
}

impl<K, V, S> CollectionRef for DashMap<K, V, S>
		where K: Eq + Hash, S: BuildHasher {
	type ItemRef<'a> where Self: 'a = Ref<'a, K, V, S>;
}

impl<K, V, S> CollectionLock for DashMap<K, V, S>
		where K: Eq + Hash, S: BuildHasher {
	type ItemGuard<'a> where Self: 'a = RefMut<'a, K, V, S>;
}

impl<K, V, S> Len for DashMap<K, V, S>
		where K: Eq + Hash, S: BuildHasher + Clone {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}
}

impl<'q, Q, K, V, S> Get<&'q Q> for DashMap<K, V, S>
		where K: Eq + Hash + Borrow<Q>, S: BuildHasher + Clone, Q: Hash + Eq {
	#[inline(always)]
	fn get<'a>(&'a self, key: &'q Q) -> Option<Ref<'a, K, V, S>> {
		self.get(key)
	}
}

impl<'q, Q, K, V, S> GetLock<&'q Q> for DashMap<K, V, S>
		where K: Eq + Hash + Borrow<Q>, S: BuildHasher + Clone, Q: Hash + Eq {
	#[inline(always)]
	fn get_lock<'a>(&'a self, key: &'q Q) -> Option<RefMut<'a, K, V, S>> {
		self.get_mut(key)
	}
}

impl<K, V, S> MapInsertLock<K> for DashMap<K, V, S>
		where K: Eq + Hash, S: BuildHasher + Clone {
	type Output<'a> where Self: 'a = Option<V>;

	#[inline(always)]
	fn insert_lock(&self, key: K, value: V) -> Option<V> {
		self.insert(key, value)
	}
}

impl<'q, Q, K, V, S> RemoveLock<&'q Q> for DashMap<K, V, S>
		where K: Eq + Hash + Borrow<Q>, S: BuildHasher + Clone, Q: Hash + Eq  {
	#[inline(always)]
	fn remove_lock(&self, key: &'q Q) -> Option<V> {
		self.remove(key).map(|(_, value)| value)
	}
}

impl<K, V, S> ClearLock for DashMap<K, V, S>
		where K: Eq + Hash, S: BuildHasher + Clone {
	#[inline(always)]
	fn clear_lock(&self) {
		self.clear()
	}
}
