use crate::{
	Clear, Collection, CollectionMut, CollectionRef, Entry, EntryApi, Get, GetKeyValue, GetMut,
	Iter, Keyed, KeyedRef, Len, MapInsert, MapIter, MapIterMut, OccupiedEntry, Remove, VacantEntry,
};
use std::{
	borrow::Borrow,
	collections::{hash_map, HashMap},
	hash::Hash,
};

impl<K, V> Collection for HashMap<K, V> {
	type Item = V;
}

impl<K, V> CollectionRef for HashMap<K, V> {
	type ItemRef<'a>
	where
		Self: 'a,
	= &'a V;

	crate::covariant_item_ref!();
}

impl<K, V> CollectionMut for HashMap<K, V> {
	type ItemMut<'a>
	where
		Self: 'a,
	= &'a mut V;

	crate::covariant_item_mut!();
}

impl<K, V> Keyed for HashMap<K, V> {
	type Key = K;
}

impl<K, V> KeyedRef for HashMap<K, V> {
	type KeyRef<'a>
	where
		Self: 'a,
	= &'a K;

	crate::covariant_key_ref!();
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

impl<'a, Q, K: Hash + Eq, V> Get<&'a Q> for HashMap<K, V>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get(&self, key: &'a Q) -> Option<&V> {
		self.get(key)
	}
}

impl<'a, Q, K: Hash + Eq, V> GetMut<&'a Q> for HashMap<K, V>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get_mut(&mut self, key: &'a Q) -> Option<&mut V> {
		self.get_mut(key)
	}
}

impl<'a, Q, K: Hash + Eq, V> GetKeyValue<&'a Q> for HashMap<K, V>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get_key_value(&self, key: &'a Q) -> Option<(&K, &V)> {
		self.get_key_value(key)
	}
}

impl<K: Hash + Eq, V> MapInsert<K> for HashMap<K, V> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: V) -> Option<V> {
		self.insert(key, value)
	}
}

impl<'a, Q, K: Hash + Eq, V> Remove<&'a Q> for HashMap<K, V>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
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

impl<K, V> Iter for HashMap<K, V> {
	type Iter<'a>
	where
		Self: 'a,
	= std::collections::hash_map::Values<'a, K, V>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}

impl<K, V> MapIter for HashMap<K, V> {
	type Iter<'a>
	where
		Self: 'a,
	= std::collections::hash_map::Iter<'a, K, V>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<K, V> MapIterMut for HashMap<K, V> {
	type IterMut<'a>
	where
		Self: 'a,
	= std::collections::hash_map::IterMut<'a, K, V>;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}

impl<'a, K, V> OccupiedEntry<'a> for hash_map::OccupiedEntry<'a, K, V> {
	type K = K;
	type V = V;

	#[inline(always)]
	fn key(&self) -> &Self::K {
		hash_map::OccupiedEntry::key(self)
	}

	#[inline(always)]
	fn remove_entry(self) -> (Self::K, Self::V) {
		hash_map::OccupiedEntry::remove_entry(self)
	}

	#[inline(always)]
	fn get(&self) -> &Self::V {
		hash_map::OccupiedEntry::get(self)
	}

	#[inline(always)]
	fn get_mut(&mut self) -> &mut Self::V {
		hash_map::OccupiedEntry::get_mut(self)
	}

	#[inline(always)]
	fn into_mut(self) -> &'a mut Self::V {
		hash_map::OccupiedEntry::into_mut(self)
	}

	#[inline(always)]
	fn insert(&mut self, value: Self::V) -> Self::V {
		hash_map::OccupiedEntry::insert(self, value)
	}

	#[inline(always)]
	fn remove(self) -> Self::V {
		hash_map::OccupiedEntry::remove(self)
	}
}

impl<'a, K, V> VacantEntry<'a> for hash_map::VacantEntry<'a, K, V> {
	type K = K;
	type V = V;

	#[inline(always)]
	fn key(&self) -> &Self::K {
		hash_map::VacantEntry::key(self)
	}

	#[inline(always)]
	fn into_key(self) -> Self::K {
		hash_map::VacantEntry::into_key(self)
	}

	#[inline(always)]
	fn insert(self, value: Self::V) -> &'a mut Self::V {
		hash_map::VacantEntry::insert(self, value)
	}
}

impl<K: Hash + Eq, V> EntryApi for HashMap<K, V> {
	type Occ<'a>
	where
		Self: 'a
	= hash_map::OccupiedEntry<'a, K, V>;
	type Vac<'a>
	where
		Self: 'a
	= hash_map::VacantEntry<'a, K, V>;

	#[inline(always)]
	fn entry(&mut self, key: Self::Key) -> Entry<Self::Occ<'_>, Self::Vac<'_>> {
		match HashMap::entry(self, key) {
			hash_map::Entry::Occupied(o) => Entry::Occupied(o),
			hash_map::Entry::Vacant(v) => Entry::Vacant(v),
		}
	}
}
