use crate::{Clear, Collection, CollectionMut, CollectionRef, Entry, EntryApi, Get, GetKeyValue, GetMut, Iter, Keyed, KeyedRef, KeyVacantEntry, Len, MapInsert, MapIter, MapIterMut, OccupiedEntry, Remove, VacantEntry};
use std::{
	borrow::Borrow,
	collections::{btree_map, BTreeMap},
};

impl<K, V> Collection for BTreeMap<K, V> {
	type Item = V;
}

impl<K, V> CollectionRef for BTreeMap<K, V> {
	type ItemRef<'a>
	where
		Self: 'a,
	= &'a V;

	crate::covariant_item_ref!();
}

impl<K, V> CollectionMut for BTreeMap<K, V> {
	type ItemMut<'a>
	where
		Self: 'a,
	= &'a mut V;

	crate::covariant_item_mut!();
}

impl<K, V> Keyed for BTreeMap<K, V> {
	type Key = K;
}

impl<K, V> KeyedRef for BTreeMap<K, V> {
	type KeyRef<'a>
	where
		Self: 'a,
	= &'a K;

	crate::covariant_key_ref!();
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

impl<'a, Q, K: Ord, V> Get<&'a Q> for BTreeMap<K, V>
where
	K: Borrow<Q>,
	Q: Ord + ?Sized,
{
	#[inline(always)]
	fn get(&self, key: &'a Q) -> Option<&V> {
		self.get(key)
	}
}

impl<'a, Q, K: Ord, V> GetKeyValue<&'a Q> for BTreeMap<K, V>
where
	K: Borrow<Q>,
	Q: Ord + ?Sized,
{
	#[inline(always)]
	fn get_key_value(&self, key: &'a Q) -> Option<(&K, &V)> {
		self.get_key_value(key)
	}
}

impl<'a, Q, K: Ord, V> GetMut<&'a Q> for BTreeMap<K, V>
where
	K: Borrow<Q>,
	Q: Ord + ?Sized,
{
	#[inline(always)]
	fn get_mut(&mut self, key: &'a Q) -> Option<&mut V> {
		self.get_mut(key)
	}
}

impl<K: Ord, V> MapInsert<K> for BTreeMap<K, V> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: V) -> Option<V> {
		self.insert(key, value)
	}
}

impl<'a, Q, K: Ord, V> Remove<&'a Q> for BTreeMap<K, V>
where
	K: Borrow<Q>,
	Q: Ord + ?Sized,
{
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

impl<K, V> Iter for BTreeMap<K, V> {
	type Iter<'a>
	where
		Self: 'a,
	= std::collections::btree_map::Values<'a, K, V>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}

impl<K, V> MapIter for BTreeMap<K, V> {
	type Iter<'a>
	where
		Self: 'a,
	= std::collections::btree_map::Iter<'a, K, V>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<K, V> MapIterMut for BTreeMap<K, V> {
	type IterMut<'a>
	where
		Self: 'a,
	= std::collections::btree_map::IterMut<'a, K, V>;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}

impl<'a, K: Ord, V> OccupiedEntry<'a> for btree_map::OccupiedEntry<'a, K, V> {
	type K = K;
	type V = V;

	#[inline(always)]
	fn key(&self) -> &Self::K {
		btree_map::OccupiedEntry::key(self)
	}

	#[inline(always)]
	fn remove_entry(self) -> (Self::K, Self::V) {
		btree_map::OccupiedEntry::remove_entry(self)
	}

	#[inline(always)]
	fn get(&self) -> &Self::V {
		btree_map::OccupiedEntry::get(self)
	}

	#[inline(always)]
	fn get_mut(&mut self) -> &mut Self::V {
		btree_map::OccupiedEntry::get_mut(self)
	}

	#[inline(always)]
	fn into_mut(self) -> &'a mut Self::V {
		btree_map::OccupiedEntry::into_mut(self)
	}

	#[inline(always)]
	fn insert(&mut self, value: Self::V) -> Self::V {
		btree_map::OccupiedEntry::insert(self, value)
	}

	#[inline(always)]
	fn remove(self) -> Self::V {
		btree_map::OccupiedEntry::remove(self)
	}
}

impl<'a, K: Ord, V> VacantEntry<'a> for btree_map::VacantEntry<'a, K, V> {
	type K = K;
	type V = V;

	#[inline(always)]
	fn into_key(self) -> Self::K {
		btree_map::VacantEntry::into_key(self)
	}

	#[inline(always)]
	fn insert(self, value: Self::V) -> &'a mut Self::V {
		btree_map::VacantEntry::insert(self, value)
	}
}

impl<'a, K: Ord, V> KeyVacantEntry<'a> for btree_map::VacantEntry<'a, K, V> {
	#[inline(always)]
	fn key(&self) -> &Self::K {
		btree_map::VacantEntry::key(self)
	}
}

impl<K: Ord, V> EntryApi for BTreeMap<K, V> {
	type Occ<'a>
	where
		Self: 'a,
	= btree_map::OccupiedEntry<'a, K, V>;
	type Vac<'a>
	where
		Self: 'a,
	= btree_map::VacantEntry<'a, K, V>;

	#[inline(always)]
	fn entry(&mut self, key: Self::Key) -> Entry<Self::Occ<'_>, Self::Vac<'_>> {
		match BTreeMap::entry(self, key) {
			btree_map::Entry::Occupied(o) => Entry::Occupied(o),
			btree_map::Entry::Vacant(v) => Entry::Vacant(v),
		}
	}
}

#[cfg(feature="raw-api")]
impl<'a, K, V> OccupiedEntry<'a> for btree_map::RawOccupiedEntryMut<'a, K, V> {
	type K = K;
	type V = V;

	#[inline(always)]
	fn key(&self) -> &Self::K {
		btree_map::RawOccupiedEntryMut::key(self)
	}

	#[inline(always)]
	fn remove_entry(self) -> (Self::K, Self::V) {
		btree_map::RawOccupiedEntryMut::remove_entry(self)
	}

	#[inline(always)]
	fn get(&self) -> &Self::V {
		btree_map::RawOccupiedEntryMut::get(self)
	}

	#[inline(always)]
	fn get_mut(&mut self) -> &mut Self::V {
		btree_map::RawOccupiedEntryMut::get_mut(self)
	}

	#[inline(always)]
	fn into_mut(self) -> &'a mut Self::V {
		btree_map::RawOccupiedEntryMut::into_mut(self)
	}

	#[inline(always)]
	fn insert(&mut self, value: Self::V) -> Self::V {
		btree_map::RawOccupiedEntryMut::insert(self, value)
	}

	#[inline(always)]
	fn remove(self) -> Self::V {
		btree_map::RawOccupiedEntryMut::remove(self)
	}
}


#[cfg(feature="raw-api")]
impl<'a, K, V> crate::RawVacantEntry<'a> for btree_map::RawOccupiedEntryMut<'a, K, V> {
	type K = K;
	type V = V;

	fn insert(self, key: Self::K, value: Self::V) -> (&'a mut Self::K, &'a mut Self::V) {
		btree_map::RawOccupiedEntryMut::insert(self, key, value)
	}
}


#[cfg(feature="raw-api")]
impl<Q: Ord + ToOwned<Owned = K>, K: Ord, V> crate::EntryRefApi<Q> for BTreeMap<K, V>{
	type Occ<'a> where Self: 'a = btree_map::RawOccupiedEntryMut<'a, K, V>;
	type Vac<'a, 'b> where Self: 'a = crate::KeyedRawVacantEntry<'a, 'b, Q, btree_map::RawVacantEntryMut<'a, K, V>>;

	fn entry_ref<'a, 'b: 'a>(&'a mut self, key: &'b Q) -> Entry<Self::Occ<'a>, Self::Vac<'a, 'b>> {
		let mut raw = self.raw_entry_mut();
		match raw.from_key(key) {
			btree_map::RawEntryMut::Occupied(occ) => {Entry::Occupied(occ)}
			btree_map::RawEntryMut::Vacant(vac) => {
				Entry::Vacant(crate::KeyedRawVacantEntry{key, raw: vac})
			}
		}
	}
}