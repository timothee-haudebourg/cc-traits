use crate::{
	Capacity, Clear, Collection, CollectionMut, CollectionRef, Get, GetKeyValue, GetKeyValueMut,
	GetMut, Iter, Keyed, KeyedRef, Len, MapInsert, MapIter, MapIterMut, PopBack, Remove, Reserve,
	SimpleCollectionMut, SimpleCollectionRef, WithCapacity,
};
use core::hash::{BuildHasher, Hash};
use indexmap::{Equivalent, IndexMap};

impl<K, V, S> Collection for IndexMap<K, V, S> {
	type Item = V;
}

impl<K, V, S> CollectionRef for IndexMap<K, V, S> {
	type ItemRef<'a> = &'a V where Self: 'a;

	crate::covariant_item_ref!();
}

impl<K, V, S> CollectionMut for IndexMap<K, V, S> {
	type ItemMut<'a> = &'a mut V where Self: 'a;

	crate::covariant_item_mut!();
}

impl<K, V, S> SimpleCollectionRef for IndexMap<K, V, S> {
	fn into_ref<'a>(r: &'a V) -> &'a V
	where
		Self: 'a,
	{
		r
	}
}

impl<K, V, S> SimpleCollectionMut for IndexMap<K, V, S> {
	fn into_mut<'a>(r: &'a mut V) -> &'a mut V
	where
		Self: 'a,
	{
		r
	}
}

impl<K, V, S> Keyed for IndexMap<K, V, S> {
	type Key = K;
}

impl<K, V, S> KeyedRef for IndexMap<K, V, S> {
	type KeyRef<'a> = &'a K where Self: 'a;

	crate::covariant_key_ref!();
}

impl<K, V, S> Len for IndexMap<K, V, S> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<'a, Q, K, V, S: BuildHasher> Get<&'a Q> for IndexMap<K, V, S>
where
	K: Eq + Hash,
	Q: Hash + Equivalent<K> + ?Sized,
{
	#[inline(always)]
	fn get(&self, key: &'a Q) -> Option<&V> {
		self.get(key)
	}
}

impl<'a, Q, K, V, S: BuildHasher> GetMut<&'a Q> for IndexMap<K, V, S>
where
	K: Hash + Eq,
	Q: Hash + Equivalent<K> + ?Sized,
{
	#[inline(always)]
	fn get_mut(&mut self, key: &'a Q) -> Option<&mut V> {
		self.get_mut(key)
	}
}

impl<'a, Q, K, V, S: BuildHasher> GetKeyValue<&'a Q> for IndexMap<K, V, S>
where
	K: Eq + Hash,
	Q: Hash + Equivalent<K> + ?Sized,
{
	#[inline(always)]
	fn get_key_value(&self, key: &'a Q) -> Option<(&K, &V)> {
		self.get_key_value(key)
	}
}

impl<K: Hash + Eq, V, S: BuildHasher> MapInsert<K> for IndexMap<K, V, S> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: V) -> Option<V> {
		self.insert(key, value)
	}
}

/// This implementation uses the `IndexMap::swap_remove` function.
impl<'a, Q, K: Hash + Eq, V, S: BuildHasher> Remove<&'a Q> for IndexMap<K, V, S>
where
	Q: Hash + Equivalent<K> + ?Sized,
{
	#[inline(always)]
	fn remove(&mut self, key: &'a Q) -> Option<V> {
		self.swap_remove(key)
	}
}

/// Remove an element based on an index. Uses `IndexMap::swap_remove_index` under the hood.
impl<K, V, S> Remove<usize> for IndexMap<K, V, S> {
	#[inline(always)]
	fn remove(&mut self, idx: usize) -> Option<V> {
		self.swap_remove_index(idx).map(|x| x.1)
	}
}

impl<K, V, S> Clear for IndexMap<K, V, S> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<K, V, S> Iter for IndexMap<K, V, S> {
	type Iter<'a> = indexmap::map::Values<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}

impl<K, V, S> MapIter for IndexMap<K, V, S> {
	type Iter<'a> = indexmap::map::Iter<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<K, V, S> MapIterMut for IndexMap<K, V, S> {
	type IterMut<'a> = indexmap::map::IterMut<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}

impl<K, V, S> Capacity for IndexMap<K, V, S> {
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

impl<K: Eq + Hash, V, S: BuildHasher> Reserve for IndexMap<K, V, S> {
	#[inline(always)]
	fn reserve(&mut self, additional: usize) {
		IndexMap::reserve(self, additional)
	}
}

impl<K: Eq + Hash, V, S: BuildHasher> PopBack for IndexMap<K, V, S> {
	#[inline(always)]
	fn pop_back(&mut self) -> Option<V> {
		self.pop().map(|(_, v)| v)
	}
}

impl<K, V, S> Get<usize> for IndexMap<K, V, S> {
	#[inline(always)]
	fn get(&self, idx: usize) -> Option<&V> {
		self.get_index(idx).map(|(_, v)| v)
	}
}

impl<K, V, S> GetMut<usize> for IndexMap<K, V, S> {
	#[inline(always)]
	fn get_mut(&mut self, idx: usize) -> Option<&mut V> {
		self.get_index_mut(idx).map(|(_, v)| v)
	}
}

impl<K, V, S> GetKeyValue<usize> for IndexMap<K, V, S> {
	#[inline(always)]
	fn get_key_value(&self, idx: usize) -> Option<(&K, &V)> {
		self.get_index(idx)
	}
}

impl<'a, Q, K, V, S> GetKeyValueMut<&'a Q> for IndexMap<K, V, S>
where
	K: Hash + Eq,
	Q: Hash + Equivalent<K>,
	S: BuildHasher,
{
	#[inline(always)]
	fn get_key_value_mut(&mut self, key: &Q) -> Option<(&K, &mut V)> {
		self.get_full_mut(key).map(|(_idx, k, v)| (k, v))
	}
}

impl<K, V, S> GetKeyValueMut<usize> for IndexMap<K, V, S> {
	#[inline(always)]
	fn get_key_value_mut(&mut self, idx: usize) -> Option<(&K, &mut V)> {
		self.get_index_mut(idx).map(|(k, v)| (&*k, v))
	}
}
