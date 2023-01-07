use crate::{
	Capacity, Clear, Collection, CollectionMut, CollectionRef, Get, GetKeyValue, GetMut, Iter,
	Keyed, KeyedRef, Len, MapInsert, MapIter, MapIterMut, PopBack, Remove, Reserve,
	SimpleCollectionMut, SimpleCollectionRef, WithCapacity,
};
use indexmap::IndexMap;
use std::{borrow::Borrow, hash::Hash};

impl<K, V> Collection for IndexMap<K, V> {
	type Item = V;
}

impl<K, V> CollectionRef for IndexMap<K, V> {
	type ItemRef<'a> = &'a V where Self: 'a;

	crate::covariant_item_ref!();
}

impl<K, V> CollectionMut for IndexMap<K, V> {
	type ItemMut<'a> = &'a mut V where Self: 'a;

	crate::covariant_item_mut!();
}

impl<K, V> SimpleCollectionRef for IndexMap<K, V> {
	fn into_ref<'a>(r: &'a V) -> &'a V
	where
		Self: 'a,
	{
		r
	}
}

impl<K, V> SimpleCollectionMut for IndexMap<K, V> {
	fn into_mut<'a>(r: &'a mut V) -> &'a mut V
	where
		Self: 'a,
	{
		r
	}
}

impl<K, V> Keyed for IndexMap<K, V> {
	type Key = K;
}

impl<K, V> KeyedRef for IndexMap<K, V> {
	type KeyRef<'a> = &'a K where Self: 'a;

	crate::covariant_key_ref!();
}

impl<K, V> Len for IndexMap<K, V> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<'a, Q, K: Hash + Eq, V> Get<&'a Q> for IndexMap<K, V>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get(&self, key: &'a Q) -> Option<&V> {
		self.get(key)
	}
}

impl<'a, Q, K: Hash + Eq, V> GetMut<&'a Q> for IndexMap<K, V>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get_mut(&mut self, key: &'a Q) -> Option<&mut V> {
		self.get_mut(key)
	}
}

impl<'a, Q, K: Hash + Eq, V> GetKeyValue<&'a Q> for IndexMap<K, V>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get_key_value(&self, key: &'a Q) -> Option<(&K, &V)> {
		self.get_key_value(key)
	}
}

impl<K: Hash + Eq, V> MapInsert<K> for IndexMap<K, V> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: V) -> Option<V> {
		self.insert(key, value)
	}
}

impl<'a, Q, K: Hash + Eq, V> Remove<&'a Q> for IndexMap<K, V>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn remove(&mut self, key: &'a Q) -> Option<V> {
		self.remove(key)
	}
}

impl<K, V> Clear for IndexMap<K, V> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<K, V> Iter for IndexMap<K, V> {
	type Iter<'a> = indexmap::map::Values<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}

impl<K, V> MapIter for IndexMap<K, V> {
	type Iter<'a> = indexmap::map::Iter<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<K, V> MapIterMut for IndexMap<K, V> {
	type IterMut<'a> = indexmap::map::IterMut<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}

impl<K, V> Capacity for IndexMap<K, V> {
	#[inline(always)]
	fn capacity(&self) -> usize {
		self.capacity()
	}
}

impl<K, V> WithCapacity for IndexMap<K, V> {
	#[inline(always)]
	fn with_capacity(capacity: usize) -> Self {
		IndexMap::with_capacity(capacity)
	}
}

impl<K: Eq + Hash, V> Reserve for IndexMap<K, V> {
	#[inline(always)]
	fn reserve(&mut self, additional: usize) {
		IndexMap::reserve(self, additional)
	}
}

impl<K: Eq + Hash, V> PopBack for IndexMap<K, V> {
	#[inline(always)]
	fn pop_back(&mut self) -> Option<V> {
		self.pop().map(|(_, v)| v)
	}
}

impl<K: Hash + Eq, V> Get<usize> for IndexMap<K, V> {
	#[inline(always)]
	fn get(&self, idx: usize) -> Option<&V> {
		self.get_index(idx).map(|(_, v)| v)
	}
}

impl<K: Hash + Eq, V> GetMut<usize> for IndexMap<K, V> {
	#[inline(always)]
	fn get_mut(&mut self, idx: usize) -> Option<&mut V> {
		self.get_index_mut(idx).map(|(_, v)| v)
	}
}

impl<K: Hash + Eq, V> GetKeyValue<usize> for IndexMap<K, V> {
	#[inline(always)]
	fn get_key_value(&self, idx: usize) -> Option<(&K, &V)> {
		self.get_index(idx)
	}
}
