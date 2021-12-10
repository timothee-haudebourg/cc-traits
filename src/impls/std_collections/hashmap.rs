use crate::{Clear, Collection, CollectionMut, CollectionRef, Entry, EntryApi, Get, GetKeyValue, GetMut, Iter, Keyed, KeyedRef, KeyVacantEntry, Len, MapInsert, MapIter, MapIterMut, OccupiedEntry, Remove, VacantEntry};
use std::{
	borrow::Borrow,
	collections::{hash_map as this_mod, HashMap as ThisMap},
	hash::{BuildHasher, Hash},
};

impl<K, V, S: BuildHasher> Collection for ThisMap<K, V, S> {
	type Item = V;
}

impl<K, V, S: BuildHasher> CollectionRef for ThisMap<K, V, S> {
	type ItemRef<'a>
	where
		Self: 'a,
	= &'a V;

	crate::covariant_item_ref!();
}

impl<K, V, S: BuildHasher> CollectionMut for ThisMap<K, V, S> {
	type ItemMut<'a>
	where
		Self: 'a,
	= &'a mut V;

	crate::covariant_item_mut!();
}

impl<K, V, S: BuildHasher> Keyed for ThisMap<K, V, S> {
	type Key = K;
}

impl<K, V, S: BuildHasher> KeyedRef for ThisMap<K, V, S> {
	type KeyRef<'a>
	where
		Self: 'a,
	= &'a K;

	crate::covariant_key_ref!();
}

impl<K, V, S: BuildHasher> Len for ThisMap<K, V, S> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<'a, Q, K: Hash + Eq, V, S: BuildHasher> Get<&'a Q> for ThisMap<K, V, S>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get(&self, key: &'a Q) -> Option<&V> {
		self.get(key)
	}
}

impl<'a, Q, K: Hash + Eq, V, S: BuildHasher> GetMut<&'a Q> for ThisMap<K, V, S>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get_mut(&mut self, key: &'a Q) -> Option<&mut V> {
		self.get_mut(key)
	}
}

impl<'a, Q, K: Hash + Eq, V, S: BuildHasher> GetKeyValue<&'a Q> for ThisMap<K, V, S>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get_key_value(&self, key: &'a Q) -> Option<(&K, &V)> {
		self.get_key_value(key)
	}
}

impl<K: Hash + Eq, V, S: BuildHasher> MapInsert<K> for ThisMap<K, V, S> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: V) -> Option<V> {
		self.insert(key, value)
	}
}

impl<'a, Q, K: Hash + Eq, V, S: BuildHasher> Remove<&'a Q> for ThisMap<K, V, S>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn remove(&mut self, key: &'a Q) -> Option<V> {
		self.remove(key)
	}
}

impl<K, V, S: BuildHasher> Clear for ThisMap<K, V, S> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<K, V, S: BuildHasher> Iter for ThisMap<K, V, S> {
	type Iter<'a>
	where
		Self: 'a,
	= this_mod::Values<'a, K, V>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}

impl<K, V, S: BuildHasher> MapIter for ThisMap<K, V, S> {
	type Iter<'a>
	where
		Self: 'a,
	= this_mod::Iter<'a, K, V>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<K, V, S: BuildHasher> MapIterMut for ThisMap<K, V, S> {
	type IterMut<'a>
	where
		Self: 'a,
	= this_mod::IterMut<'a, K, V>;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}

impl<'a, K, V> OccupiedEntry<'a> for this_mod::OccupiedEntry<'a, K, V> {
	type K = K;
	type V = V;

	#[inline(always)]
	fn key(&self) -> &Self::K {
		this_mod::OccupiedEntry::key(self)
	}

	#[inline(always)]
	fn remove_entry(self) -> (Self::K, Self::V) {
		this_mod::OccupiedEntry::remove_entry(self)
	}

	#[inline(always)]
	fn get(&self) -> &Self::V {
		this_mod::OccupiedEntry::get(self)
	}

	#[inline(always)]
	fn get_mut(&mut self) -> &mut Self::V {
		this_mod::OccupiedEntry::get_mut(self)
	}

	#[inline(always)]
	fn into_mut(self) -> &'a mut Self::V {
		this_mod::OccupiedEntry::into_mut(self)
	}

	#[inline(always)]
	fn insert(&mut self, value: Self::V) -> Self::V {
		this_mod::OccupiedEntry::insert(self, value)
	}

	#[inline(always)]
	fn remove(self) -> Self::V {
		this_mod::OccupiedEntry::remove(self)
	}
}

impl<'a, K, V> VacantEntry<'a> for this_mod::VacantEntry<'a, K, V> {
	type K = K;
	type V = V;

	#[inline(always)]
	fn into_key(self) -> Self::K {
		this_mod::VacantEntry::into_key(self)
	}

	#[inline(always)]
	fn insert(self, value: Self::V) -> &'a mut Self::V {
		this_mod::VacantEntry::insert(self, value)
	}
}

impl<'a, K, V> KeyVacantEntry<'a> for this_mod::VacantEntry<'a, K, V> {
	#[inline(always)]
	fn key(&self) -> &Self::K {
		this_mod::VacantEntry::key(self)
	}
}

impl<K: Hash + Eq, V, S: BuildHasher> EntryApi for ThisMap<K, V, S> {
	type Occ<'a>
	where
		Self: 'a,
	= this_mod::OccupiedEntry<'a, K, V>;
	type Vac<'a>
	where
		Self: 'a,
	= this_mod::VacantEntry<'a, K, V>;

	#[inline(always)]
	fn entry(&mut self, key: Self::Key) -> Entry<Self::Occ<'_>, Self::Vac<'_>> {
		match ThisMap::entry(self, key) {
			this_mod::Entry::Occupied(o) => Entry::Occupied(o),
			this_mod::Entry::Vacant(v) => Entry::Vacant(v),
		}
	}
}

#[cfg(feature="raw-api")]
impl<'a, K, V, S: BuildHasher> OccupiedEntry<'a> for this_mod::RawOccupiedEntryMut<'a, K, V, S> {
	type K = K;
	type V = V;

	#[inline(always)]
	fn key(&self) -> &Self::K {
		this_mod::RawOccupiedEntryMut::key(self)
	}

	#[inline(always)]
	fn remove_entry(self) -> (Self::K, Self::V) {
		this_mod::RawOccupiedEntryMut::remove_entry(self)
	}

	#[inline(always)]
	fn get(&self) -> &Self::V {
		this_mod::RawOccupiedEntryMut::get(self)
	}

	#[inline(always)]
	fn get_mut(&mut self) -> &mut Self::V {
		this_mod::RawOccupiedEntryMut::get_mut(self)
	}

	#[inline(always)]
	fn into_mut(self) -> &'a mut Self::V {
		this_mod::RawOccupiedEntryMut::into_mut(self)
	}

	#[inline(always)]
	fn insert(&mut self, value: Self::V) -> Self::V {
		this_mod::RawOccupiedEntryMut::insert(self, value)
	}

	#[inline(always)]
	fn remove(self) -> Self::V {
		this_mod::RawOccupiedEntryMut::remove(self)
	}
}


#[cfg(feature="raw-api")]
impl<'a, K, V, S: BuildHasher> crate::RawVacantEntry<'a> for this_mod::RawOccupiedEntryMut<'a, K, V, S> {
	type K = K;
	type V = V;

	fn insert(self, key: Self::K, value: Self::V) -> (&'a mut Self::K, &'a mut Self::V) {
		this_mod::RawOccupiedEntryMut::insert(self, key, value)
	}
}


#[cfg(feature="raw-api")]
impl<Q: Hash + Eq + ToOwned<Owned = K>, K: Hash + Eq, V, S: BuildHasher> crate::EntryRefApi<Q> for ThisMap<K, V, S>{
	type Occ<'a> where Self: 'a = this_mod::RawOccupiedEntryMut<'a, K, V, S>;
	type Vac<'a, 'b> where Self: 'a = crate::KeyedRawVacantEntry<'a, 'b, Q, this_mod::RawVacantEntryMut<'a, K, V, S>>;

	fn entry_ref<'a, 'b: 'a>(&'a mut self, key: &'b Q) -> Entry<Self::Occ<'a>, Self::Vac<'a, 'b>> {
		let mut raw = self.raw_entry_mut();
		match raw.from_key(key) {
			this_mod::RawEntryMut::Occupied(occ) => {Entry::Occupied(occ)}
			this_mod::RawEntryMut::Vacant(vac) => {
				Entry::Vacant(crate::KeyedRawVacantEntry{key, raw: vac})
			}
		}
	}
}